pub mod course;

use clap::Subcommand;
use gix::{Repository, ThreadSafeRepository};
use miette::{IntoDiagnostic, Result};

use crate::{
    Cli,
    errors::{NoGitRepo, NoGitWorkdir, NoManifest},
    git::{RepoType, detect_repo_type},
};

#[derive(Subcommand, Debug, Clone)]
pub enum SyncCommands {
    /// Synchronize a course
    Course,
}

pub async fn handle_apply(cli: &Cli) -> Result<()> {
    let repo: Repository = ThreadSafeRepository::discover(".")
        .map_err(|_| NoGitRepo)?
        .into();

    let local_repo = repo.clone();
    let directory = local_repo.workdir().ok_or(NoGitWorkdir)?;
    let repo_type = detect_repo_type(directory).await;

    match repo_type {
        RepoType::Course => course::run(repo, directory).await,
        RepoType::Challenge => todo!(),
        RepoType::Unknown => Err(NoManifest.into()),
    }
}
