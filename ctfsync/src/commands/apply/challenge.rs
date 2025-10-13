use std::path::Path;

use api_client::{
    apis::challenge_api,
    models::{ChallengeMetadata, UpdateChallengeRequest},
};
use gix::Repository;
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use serde::Deserialize;

use crate::{api::init_api_config, errors::NoManifest};

pub async fn run(_repo: Repository, directory: &Path) -> Result<()> {
    // TODO: Integrate with Git
    println!("Applying {}", "challenge.yaml".yellow().bold());

    let manifest = load_challenge_manifest::<ChallengeMetadata>(directory).await?;

    let config = init_api_config().await?;

    // TODO: Handle refs properly
    let update_request = UpdateChallengeRequest::new(
        manifest.clone(),
        "8a3920b7b28274305c1ee2182d4aff0e277f6fd2".to_string(),
    );
    challenge_api::update_challenge(config, &manifest.slug, update_request)
        .await
        .into_diagnostic()?;

    Ok(())
}

pub async fn load_challenge_manifest<T>(directory: &Path) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let manifest = directory.join("challenge.yaml");
    let manifest = tokio::fs::read_to_string(manifest)
        .await
        .map_err(|_| NoManifest)?;
    let manifest = serde_yaml::from_str::<T>(&manifest).into_diagnostic()?;
    Ok(manifest)
}
