use std::path::Path;

use api_client::{
    apis::courses_api,
    models::{CourseMetadata, UpdateCourseRequest},
};
use gix::Repository;
use miette::{IntoDiagnostic, Result};

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

    let update_request = UpdateCourseRequest::new(manifest.clone(), r#ref);
    let update_response = courses_api::update_course(config, &manifest.slug, update_request)
        .await
        .into_diagnostic()?;

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
