use std::path::Path;

use api_client::{
    apis::courses_api,
    models::{self, UpdateCourseRequest},
};
use gix::Repository;
use manifests::CourseMetadata;
use miette::{IntoDiagnostic, Result};

use crate::{api::init_api_config, errors::NoManifest};

pub async fn run(repo: Repository, directory: &Path) -> Result<()> {
    let mut head = repo.head().into_diagnostic()?;
    let r#ref = head
        .peel_to_commit_in_place()
        .into_diagnostic()?
        .id
        .to_string();

    // Loading manifest
    let manifest = directory.join("course.yaml");
    let manifest = tokio::fs::read_to_string(manifest)
        .await
        .map_err(|_| NoManifest)?;
    let manifest = serde_yaml::from_str::<CourseMetadata>(&manifest).into_diagnostic()?;

    let config = init_api_config().await?;

    let update_request = UpdateCourseRequest::new(
        models::CourseMetadata {
            description: manifest.description,
            difficulty: match manifest.difficulty {
                manifests::CourseDifficulty::Advanced => models::CourseDifficulty::Advanced,
                manifests::CourseDifficulty::Beginner => models::CourseDifficulty::Beginner,
                manifests::CourseDifficulty::Intermediate => models::CourseDifficulty::Intermediate,
            },
            name: manifest.name,
            objectives: Some(manifest.objectives),
            prerequisites: Some(manifest.prerequisites),
            slug: manifest.slug.clone(),
            tags: Some(manifest.tags),
        },
        r#ref,
    );
    let update_response = courses_api::update_course(config, &manifest.slug, update_request)
        .await
        .into_diagnostic()?;
    dbg!(update_response);

    Ok(())
}
