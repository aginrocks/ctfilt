pub mod challenge;
pub mod course;

use std::convert::Infallible;

use clap::Subcommand;
use gix::{Repository, ThreadSafeRepository, status};
use miette::{IntoDiagnostic, Result};

use crate::{
    errors::{DityWorktree, NoGitRepo, NoGitWorkdir, NoManifest},
    git::{RepoType, detect_repo_type},
};

#[derive(Subcommand, Debug, Clone)]
pub enum SyncCommands {
    /// Synchronize a course
    Course,
}

pub async fn handle_apply() -> Result<()> {
    let repo: Repository = ThreadSafeRepository::discover(".")
        .map_err(|_| NoGitRepo)?
        .into();

    let local_repo = repo.clone();

    if is_dirty(&local_repo)? {
        return Err(DityWorktree.into());
    }

    let directory = local_repo.workdir().ok_or(NoGitWorkdir)?;
    let repo_type = detect_repo_type(directory).await;

    match repo_type {
        RepoType::Course => course::run(repo, directory).await,
        RepoType::Challenge => challenge::run(repo, directory).await,
        RepoType::Unknown => Err(NoManifest.into()),
    }
}

// Copied from gix but takes untracked files into account
fn is_dirty(repo: &Repository) -> Result<bool> {
    {
        let head_tree_id = repo.head_tree_id().into_diagnostic()?;
        let mut index_is_dirty = false;

        // Run this first as there is a high likelihood to find something, and it's very fast.
        repo.tree_index_status(
            &head_tree_id,
            &*repo.index_or_empty().into_diagnostic()?,
            None,
            status::tree_index::TrackRenames::Disabled,
            |_, _, _| {
                index_is_dirty = true;
                Ok::<_, Infallible>(gix::diff::index::Action::Cancel)
            },
        )
        .into_diagnostic()?;
        if index_is_dirty {
            return Ok(true);
        }
    }

    Ok(repo
        .status(gix::progress::Discard)
        .into_diagnostic()?
        .untracked_files(status::UntrackedFiles::Files)
        .index_worktree_rewrites(None)
        .index_worktree_submodules(status::Submodule::AsConfigured { check_dirty: true })
        .into_index_worktree_iter(vec![])
        .into_diagnostic()?
        .take_while(Result::is_ok)
        .next()
        .is_some())
}
