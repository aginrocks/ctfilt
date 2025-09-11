mod kubernetes;
mod settings;
mod state;
mod watcher;

use std::sync::Arc;

use color_eyre::eyre::{Context, Result};
use exterminator::Exterminator;
use tracing::{info, level_filters::LevelFilter};
use tracing_error::ErrorLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{kubernetes::init_kubernetes, settings::Settings, watcher::PodWatcher};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv().ok();

    init_tracing().wrap_err("failed to set global tracing subscriber")?;

    info!(
        "Starting {} {}...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    );

    let settings = Arc::new(Settings::try_load()?);

    info!(prefix = settings.extermination.labels_prefix, "Using");

    let client = init_kubernetes(&settings).await?;

    let exterminator = Arc::new(Exterminator::new(
        client.clone(),
        settings.extermination.clone(),
    ));

    let watcher = PodWatcher::new(client, settings, exterminator);
    watcher.watch_pods().await?;

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
