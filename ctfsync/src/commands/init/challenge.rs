use std::path::Path;

use clap::Parser;
use gix::ThreadSafeRepository;
use inquire::Text;
use manifests::{ChallengeMetadata, serialize_with_schema, to_slug};
use miette::{IntoDiagnostic, Result, miette};
use tokio::{fs, task};
use tracing::warn;
use url::Url;

use crate::{config::init_config, errors::NoGitWorkdir};

#[derive(Debug, Clone, Parser)]
pub struct InitChallengeArgs {
    slug: Option<String>,

    #[arg(short = 'n', long)]
    name: Option<String>,

    #[arg(short = 'd', long)]
    description: Option<String>,
}

pub async fn run(args: InitChallengeArgs) -> Result<()> {
    let name = match args.name {
        Some(name) => name,
        None => task::spawn_blocking(|| Text::new("Name").prompt())
            .await
            .into_diagnostic()?
            .into_diagnostic()?,
    };

    let initial_slug = to_slug(&name);

    let slug = match args.slug {
        Some(slug) => slug,
        None => task::spawn_blocking(move || {
            Text::new("Slug").with_initial_value(&initial_slug).prompt()
        })
        .await
        .into_diagnostic()?
        .into_diagnostic()?,
    };

    let description = match args.description {
        Some(description) => description,
        None => task::spawn_blocking(|| Text::new("Description").prompt())
            .await
            .into_diagnostic()?
            .into_diagnostic()?,
    };

    let repo = ThreadSafeRepository::discover(".");
    let repo = match repo {
        Ok(repo) => repo.into(),
        Err(_) => {
            warn!("Git repository not found, initializing one");
            gix::init(Path::new(".")).into_diagnostic()?
        }
    };

    let directory = repo.workdir().ok_or(NoGitWorkdir)?;
    let manifest_path = directory.join("challenge.yaml");

    let metadata = ChallengeMetadata {
        name,
        slug,
        description,
        ..Default::default()
    };

    let config = init_config().await?;
    let server_base = Url::parse(&config.base_url).into_diagnostic()?;
    let serialized_manifest = serialize_with_schema(metadata, "challenge", server_base)
        .map_err(|_| miette!("Failed to serialize manifest"))?;

    fs::write(manifest_path, serialized_manifest)
        .await
        .into_diagnostic()?;

    Ok(())
}
