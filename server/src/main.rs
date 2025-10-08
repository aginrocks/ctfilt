mod axum_error;
mod database;
mod headscale_client;
mod kubernetes;
mod middlewares;
mod mongo_id;
mod orchestrator;
mod redis_client;
mod routes;
mod settings;
mod state;
mod utils;

use std::{net::SocketAddr, ops::Deref, sync::Arc};

use axum::{
    Router,
    error_handling::HandleErrorLayer,
    extract::Extension,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use axum_oidc::{OidcAuthLayer, OidcClient, error::MiddlewareError};
use clap::Parser;
use color_eyre::Result;
use color_eyre::eyre::WrapErr;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_sessions::SessionManagerLayer;
use tower_sessions_redis_store::{RedisStore, fred::prelude::Pool};
use tracing::{Instrument, error, info, info_span, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt as _, util::SubscriberInitExt as _,
};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};

use crate::{
    database::{DatabaseStore, init_database, init_session_store},
    kubernetes::init_kubernetes,
    orchestrator::ChallengeOrchestrator,
    redis_client::init_redis,
    settings::Settings,
    state::AppState,
    utils::{FlagGenerator, create_token},
};

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct GroupClaims {}
impl axum_oidc::AdditionalClaims for GroupClaims {}
impl openidconnect::AdditionalClaims for GroupClaims {}

#[derive(Parser)]
struct Args {
    #[arg(long = "generate-token")]
    generate_token: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    dotenvy::dotenv().ok();

    let filter = match args.generate_token {
        true => LevelFilter::ERROR,
        false => LevelFilter::INFO,
    };
    init_tracing(filter).wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let settings = Arc::new(Settings::try_load()?);

    let database = init_database(&settings).await?;

    let store = DatabaseStore::new(&database);

    if args.generate_token {
        create_token(&database).await?;
        return Ok(());
    }

    let kube_client = init_kubernetes(&settings).await?;

    let headscale_config = Arc::new(headscale_client::init_headscale(&settings)?);

    let flags = Arc::new(FlagGenerator::new(
        settings.flags.secret.clone(),
        settings.flags.length,
    ));

    let orchestrator = Arc::new(ChallengeOrchestrator::new(
        kube_client.clone(),
        flags.clone(),
        headscale_config.clone(),
        settings.headscale.public_url.clone(),
        settings.extermination.clone(),
    ));

    let orchestrator_watcher = orchestrator.clone();
    tokio::spawn(async move {
        match orchestrator_watcher.watcher.watch_pods().await {
            Ok(_) => info!("Pod watcher exited"),
            Err(e) => error!(error = ?e, "Pod watcher exited with error"),
        }
    });

    let fred = init_redis(&settings).await?;

    let app_state = AppState {
        database,
        store,
        settings: settings.clone(),
        kube: kube_client,
        flags,
        orchestrator: orchestrator.clone(),
        headscale_config,
        fred: fred.clone(),
    };

    let session_layer = init_session_store(&settings, fred).await?;
    let app = init_axum(app_state, session_layer).await?;
    let listener = init_listener(&settings).await?;

    info!(
        "listening on {} ({})",
        listener
            .local_addr()
            .wrap_err("failed to get local address")?,
        settings.general.public_url
    );

    axum::serve(listener, app.into_make_service())
        .await
        .wrap_err("failed to run server")?;

    Ok(())
}

fn init_tracing(filter: LevelFilter) -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(filter.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

#[instrument(skip(state, session_layer))]
async fn init_axum(
    state: AppState,
    session_layer: SessionManagerLayer<RedisStore<Pool>>,
) -> Result<Router> {
    let app_url = format!(
        "{}/api/login/callback",
        state
            .settings
            .general
            .public_url
            .to_string()
            .trim_end_matches('/')
    );

    let mut oidc_client = OidcClient::<GroupClaims>::builder()
        .with_default_http_client()
        .with_redirect_url(app_url.parse()?)
        .with_client_id(state.settings.oidc.client_id.as_str())
        .add_scope("profile")
        .add_scope("email");

    if let Some(client_secret) = state.settings.oidc.client_secret.as_ref() {
        oidc_client = oidc_client.with_client_secret(client_secret.secret().clone());
    }

    let oidc_client = oidc_client
        .discover(state.settings.oidc.issuer.deref().clone())
        .instrument(info_span!("oidc_discover"))
        .await?
        .build();

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            error!(error = ?e, "An error occurred in OIDC auth middleware");
            e.into_response()
        }))
        .layer(OidcAuthLayer::new(oidc_client));

    let router = routes::routes();

    let (router, api) = router.with_state(state.clone()).split_for_parts();

    let openapi_prefix = "/apidoc";
    let spec_name = "/openapi.json";

    let docs = Router::new()
        .merge(Redoc::with_url("/redoc", api.clone()))
        .merge(RapiDoc::new(format!("{openapi_prefix}{spec_name}")).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api.clone()))
        .route(spec_name, get(|| async move { Json(api) }));

    let router = router
        .nest(openapi_prefix, docs)
        .layer(Extension(state))
        .layer(oidc_auth_service)
        .layer(session_layer)
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() });
    // .layer(
    //     TraceLayer::new_for_http()
    //         .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
    //         .on_response(DefaultOnResponse::new().level(Level::INFO)),
    // );

    Ok(router)
}

async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
