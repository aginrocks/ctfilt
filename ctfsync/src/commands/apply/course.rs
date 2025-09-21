mod diff;
mod map;

use std::{path::Path, str::FromStr};

use api_client::{
    apis::course_api,
    models::{CourseMetadataString, UpdateCourseRequest},
};
use gix::{
    ObjectId, Repository,
    bstr::ByteSlice,
    diff::{Options, tree_with_rewrites::Change},
};
use miette::{IntoDiagnostic, Result};
use serde::Deserialize;
use tracing::warn;

use crate::{
    api::init_api_config, commands::apply::course::diff::apply_diff, errors::NoManifest,
    utils::is_hidden,
};

pub async fn run(repo: Repository, directory: &Path) -> Result<()> {
    let mut head = repo.head().into_diagnostic()?;
    let r#ref = head
        .peel_to_commit_in_place()
        .into_diagnostic()?
        .id
        .to_string();

    let manifest = load_course_manifest::<CourseMetadataString>(directory).await?;

    let config = init_api_config().await?;

    let server_ref = course_api::get_course(config, &manifest.slug)
        .await
        .map(|course| course.r#ref);

    let server_tree = match server_ref {
        Ok(server_ref) => {
            let id = ObjectId::from_str(&server_ref).into_diagnostic()?;
            let commit = repo.find_commit(id);
            match commit {
                Ok(commit) => commit.tree().into_diagnostic()?,
                Err(_) => {
                    // Commit does not exist, possible force push
                    warn!(
                        "The server is on commit {server_ref}, which does not exist in your local repository. This may indicate a force-push or history rewrite. Data on the server will be OVERWRITTEN with your local state."
                    );
                    repo.empty_tree()
                }
            }
        }
        Err(_) => repo.empty_tree(),
    };

    let head_tree = repo.head_tree().into_diagnostic()?;

    let diff = repo
        .diff_tree_to_tree(&server_tree, &head_tree, Options::default())
        .into_diagnostic()?;

    let diff = diff
        .into_iter()
        .filter(|change| {
            change
                .location()
                .to_path()
                .is_ok_and(|path| path.is_file() && !is_hidden(path))
        })
        .collect::<Vec<Change>>();

    if diff.is_empty() {
        warn!("No changes to apply");
        return Ok(());
    }

    apply_diff(diff, directory, manifest.slug.clone()).await?;

    let update_request = UpdateCourseRequest::new(manifest.clone(), r#ref);
    course_api::update_course(config, &manifest.slug, update_request)
        .await
        .into_diagnostic()?;

    Ok(())
}

pub async fn load_course_manifest<T>(directory: &Path) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let manifest = directory.join("course.yaml");
    let manifest = tokio::fs::read_to_string(manifest)
        .await
        .map_err(|_| NoManifest)?;
    let manifest = serde_yaml::from_str::<T>(&manifest).into_diagnostic()?;
    Ok(manifest)
}
