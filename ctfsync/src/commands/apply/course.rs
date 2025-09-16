use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use api_client::{
    apis::courses_api,
    models::{CourseMetadata, UpdateCourseRequest},
};
use gix::{ObjectId, Repository};
use miette::{IntoDiagnostic, Result};
use tracing::warn;

use crate::{api::init_api_config, errors::NoManifest};

pub async fn run(repo: Repository, directory: &Path) -> Result<()> {
    let mut head = repo.head().into_diagnostic()?;
    let r#ref = head
        .peel_to_commit_in_place()
        .into_diagnostic()?
        .id
        .to_string();

    let manifest = load_manifest(directory).await?;

    let config = init_api_config().await?;

    let latest_ref = courses_api::get_course(config, &manifest.slug)
        .await
        .into_diagnostic()?
        .r#ref;

    let update_request = UpdateCourseRequest::new(manifest.clone(), r#ref);
    let update_response = courses_api::update_course(config, &manifest.slug, update_request)
        .await
        .into_diagnostic()?;

    let latest_ref_id = ObjectId::from_str(&latest_ref).into_diagnostic()?;
    let latest_ref_object = repo.find_commit(latest_ref_id);
    match latest_ref_object {
        Ok(latest_ref_object) => todo!(),
        Err(_) => {
            // Commit does not exist, possible force push
            warn!(
                "The server is on commit {latest_ref}, which does not exist in your local repository. This may indicate a force-push or history rewrite. If you continue, Data on the production server will be OVERWRITTEN with your local state."
            );
        }
    }

    dbg!(update_response);

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
