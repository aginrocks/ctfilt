use gix::Repository;
use manifests::CourseMetadata;
use miette::{IntoDiagnostic, Result};

use crate::errors::{NoCourseManifest, NoGitWorkdir};

pub async fn run(repo: Repository) -> Result<()> {
    let directory = repo.workdir().ok_or(NoGitWorkdir)?;

    // Loading manifest
    let manifest = directory.join("course.yaml");
    let manifest = tokio::fs::read_to_string(manifest)
        .await
        .map_err(|_| NoCourseManifest)?;
    let manifest = serde_yaml::from_str::<CourseMetadata>(&manifest).into_diagnostic()?;

    dbg!(manifest);
    Ok(())
}
