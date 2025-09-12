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
    token: String,

    #[arg(short = 'u', long, default_value = "https://ctf.agin.rocks")]
    url: String,
}

pub async fn run(args: LoginArgs) -> Result<()> {
    if init_config().await.is_ok() {
        warn!("You are already logged in. Overwriting your current credentials.",);
    }

    let config = create_api_config(&args.url, &args.token)
        .wrap_err("Failed to create HTTP client. Ensure that the server URL is valid.")?;

    // TODO: Validate API key

    let config = AppConfig {
        base_url: args.url.clone(),
        token: args.token.clone(),
    };

    config.save().await?;
    success!("Logged in");

    Ok(())
}
