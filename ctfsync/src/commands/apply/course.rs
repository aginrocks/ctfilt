use std::{path::Path, str::FromStr};

use api_client::{
    apis::courses_api,
    models::{CourseMetadata, UpdateCourseRequest},
};
use gix::{
    ObjectId, Repository,
    diff::{Options, tree_with_rewrites::Change},
};
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use tracing::{info, warn};

use crate::{api::init_api_config, errors::NoManifest, success};

pub async fn run(repo: Repository, directory: &Path) -> Result<()> {
    let mut head = repo.head().into_diagnostic()?;
    let r#ref = head
        .peel_to_commit_in_place()
        .into_diagnostic()?
        .id
        .to_string();

    let manifest = load_manifest(directory).await?;

    let config = init_api_config().await?;

    let server_ref = courses_api::get_course(config, &manifest.slug)
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

    // let update_request = UpdateCourseRequest::new(manifest.clone(), r#ref);
    // courses_api::update_course(config, &manifest.slug, update_request)
    //     .await
    //     .into_diagnostic()?;

    let head_tree = repo.head_tree().into_diagnostic()?;

    let diff = repo
        .diff_tree_to_tree(&server_tree, &head_tree, Options::default())
        .into_diagnostic()?;

    // dbg!(diff);
    if diff.is_empty() {
        warn!("No changes to apply");
        return Ok(());
    }

    for change in diff {
        let logged_change = match change {
            Change::Addition { .. } => change.location().green().to_string(),
            Change::Deletion { .. } => change.location().red().to_string(),
            Change::Modification { .. } | Change::Rewrite { .. } => {
                change.location().yellow().to_string()
            }
        };
        println!("Applying {}", logged_change.bold());
    }

    Ok(())
}

async fn load_manifest(directory: &Path) -> Result<CourseMetadata> {
    let manifest = directory.join("course.yaml");
    let manifest = tokio::fs::read_to_string(manifest)
        .await
        .map_err(|_| NoManifest)?;
    let manifest = serde_yaml::from_str::<CourseMetadata>(&manifest).into_diagnostic()?;
    Ok(manifest)
}
