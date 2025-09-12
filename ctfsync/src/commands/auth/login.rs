use clap::Parser;
use inquire::Text;
use miette::{Context, IntoDiagnostic, Result};
use tokio::task;
use tracing::warn;

use crate::{
    api::create_api_config,
    config::{AppConfig, init_config},
    success,
};

#[derive(Debug, Clone, Parser)]
pub struct LoginArgs {
    #[arg(short = 'u', long)]
    url: String,

    #[arg(short = 't', long)]
    token: String,
}

pub async fn run(args: LoginArgs) -> Result<()> {
    if init_config().await.is_ok() {
        warn!("You are already logged in. Overwriting your current credentials.",);
    }

    let base_url = task::spawn_blocking(|| Text::new("Server URL").prompt())
        .await
        .into_diagnostic()?
        .into_diagnostic()?;

    let config = create_api_config(&base_url, &args.token)
        .wrap_err("Failed to create HTTP client. Ensure that the server URL is valid.")?;

    // TODO: Validate API key

    let config = AppConfig {
        base_url: base_url.clone(),
        token: args.token.clone(),
    };

    config.save().await?;
    success!("Logged in");

    Ok(())
}
