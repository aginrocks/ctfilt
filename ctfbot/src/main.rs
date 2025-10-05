mod axum_error;
mod routes;
mod settings;
mod state;
pub mod utils;

use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router, response::IntoResponse, routing::IntoMakeService};
use color_eyre::{Result, eyre::Context};
use gitea_client::apis::configuration::Configuration;
use http::{HeaderMap, StatusCode, header::AUTHORIZATION};
use tokio::net::TcpListener;
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{settings::Settings, state::AppState};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let settings = Arc::new(Settings::try_load()?);

    let gitea = init_gitea(&settings)?;

    let app_state = AppState {
        settings: settings.clone(),
        gitea: Arc::new(gitea),
    };

    let app = init_axum(app_state)?;
    let listener = init_listener(&settings).await?;

    info!(
        "listening on {} ({})",
        listener
            .local_addr()
            .wrap_err("failed to get local address")?,
        settings.general.public_url
    );

    axum::serve(listener, app)
        .await
        .wrap_err("failed to run server")?;

    Ok(())
}

fn init_tracing() -> Result<()> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .with(ErrorLayer::default())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .with_env_var("RUST_LOG")
                .from_env()?,
        )
        .try_init()?;

    Ok(())
}

#[instrument(skip(state))]
fn init_axum(state: AppState) -> Result<IntoMakeService<Router>> {
    let app = routes::routes();

    let app = app
        .with_state(state.clone())
        .layer(Extension(state))
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found").into_response() })
        .into_make_service();

    Ok(app)
}

fn init_gitea(settings: &Settings) -> Result<Configuration> {
    let headers = HeaderMap::from_iter([(
        AUTHORIZATION,
        format!("Bearer {}", settings.git.token).parse()?,
    )]);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let config = Configuration {
        base_path: settings.git.url.join("/api/v1")?.to_string(),
        user_agent: Some(format!("ctfsync-cli/{}", env!("CARGO_PKG_VERSION"))),
        client,
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: Some(settings.git.token.clone()),
        api_key: None,
    };

    Ok(config)
}

async fn init_listener(settings: &Settings) -> Result<TcpListener> {
    let addr: Vec<SocketAddr> = settings.general.listen_address.clone().into();

    Ok(TcpListener::bind(addr.as_slice()).await?)
}
