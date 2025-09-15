use api_client::apis::other_api;
use clap::Parser;
use inquire::Password;
use miette::{Context, IntoDiagnostic, Result};
use tokio::task;
use tracing::warn;

use crate::{
    api::create_api_config,
    config::{AppConfig, init_config},
    errors::InvalidToken,
    success,
};

#[derive(Debug, Clone, Parser)]
pub struct LoginArgs {
    token: Option<String>,

    #[arg(short = 'u', long, default_value = "https://ctf.agin.rocks")]
    url: String,
}

pub async fn run(args: LoginArgs) -> Result<()> {
    if init_config().await.is_ok() {
        warn!("You are already logged in. Overwriting your current credentials.",);
    }

    let token = match args.token {
        Some(token) => token,
        None => task::spawn_blocking(|| {
            Password::new("Token")
                .without_confirmation()
                .with_display_mode(inquire::PasswordDisplayMode::Masked)
                .prompt()
        })
        .await
        .into_diagnostic()?
        .into_diagnostic()?,
    };

    let config = create_api_config(&args.url, &token)
        .wrap_err("Failed to create HTTP client. Ensure that the server URL is valid.")?;

    // TODO: Validate API key
    other_api::check_system_auth(&config)
        .await
        .map_err(|_| InvalidToken)?;

    let config = AppConfig {
        base_url: args.url.clone(),
        token: token.clone(),
    };

    config.save().await?;
    success!("Logged in");

    Ok(())
}
