pub mod course;

use clap::Subcommand;
use gix::{Repository, ThreadSafeRepository};
use miette::Result;

use crate::{Cli, errors::NoGitRepo};

#[derive(Subcommand, Debug, Clone)]
pub enum SyncCommands {
    /// Synchronize a course
    Course,
}

pub async fn handle_sync(cli: &Cli, cmd: SyncCommands) -> Result<()> {
    let repo = ThreadSafeRepository::discover(".")
        .map_err(|_| NoGitRepo)?
        .into();

    match cmd {
        SyncCommands::Course => course::run(repo).await,
        // AuthCommands::Logout => logout::run(cli).await,
    }
}
