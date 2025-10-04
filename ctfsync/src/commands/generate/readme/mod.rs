mod challenge;

use gix::{Repository, ThreadSafeRepository};
use miette::Result;

use crate::{
    errors::{NoGitRepo, NoGitWorkdir, NoManifest},
    git::{RepoType, detect_repo_type},
};

pub async fn run() -> Result<()> {
    let repo: Repository = ThreadSafeRepository::discover(".")
        .map_err(|_| NoGitRepo)?
        .into();

    let local_repo = repo.clone();

    let directory = local_repo.workdir().ok_or(NoGitWorkdir)?;
    let repo_type = detect_repo_type(directory).await;

    match repo_type {
        RepoType::Course => todo!(),
        RepoType::Challenge => challenge::run(directory).await,
        RepoType::Unknown => Err(NoManifest.into()),
    }
}
