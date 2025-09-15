mod api;
mod commands;
mod config;
mod errors;
mod formatter;
mod report_handler;
mod utils;

use clap::{Parser, Subcommand};
use miette::Result;
use std::process;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, prelude::*};

use crate::{report_handler::ErrorReportHandler, utils::get_render_config};

/// CTFSync for CTFILT
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Authentication commands, auth only supported with API keys
    Auth {
        #[command(subcommand)]
        subcommand: commands::auth::AuthCommands,
    },
    /// Synchronize this Git repo with the CTF platform
    Apply {
        #[command(subcommand)]
        subcommand: commands::apply::SyncCommands,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| Box::new(ErrorReportHandler::new())))?;
    inquire::set_global_render_config(get_render_config());

    // let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .event_format(formatter::EventFormatter)
                // .with_writer(indicatif_layer.get_stderr_writer())
                .with_filter(LevelFilter::INFO),
        )
        // .with(indicatif_layer)
        .init();

    let cli = Cli::parse();

    let result = match cli.clone().command {
        Commands::Auth { subcommand } => commands::auth::handle_auth(&cli, subcommand).await,
        Commands::Apply { subcommand } => commands::apply::handle_apply(&cli, subcommand).await,
    };

    if let Err(e) = result {
        eprintln!("{e:?}");
        process::exit(1);
    }

    Ok(())
}
