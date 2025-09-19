use clap::Parser;
use gix::{Repository, ThreadSafeRepository};
use inquire::Text;
use manifests::{LessonMetadata, serialize_with_schema, to_slug};
use miette::{IntoDiagnostic, Result, miette};
use owo_colors::OwoColorize;
use tokio::{fs, task};
use url::Url;

use crate::{
    config::init_config,
    errors::{NoGitRepo, NoGitWorkdir},
};

#[derive(Debug, Clone, Parser)]
pub struct InitLessonArgs {
    slug: Option<String>,

    #[arg(short = 'n', long)]
    name: Option<String>,
}

pub async fn run(args: InitLessonArgs) -> Result<()> {
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

    let repo: Repository = ThreadSafeRepository::discover(".")
        .map_err(|_| NoGitRepo)?
        .into();

    let directory = repo.workdir().ok_or(NoGitWorkdir)?;

    let last_index = std::fs::read_dir(directory)
        .into_diagnostic()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_dir())
        .filter_map(|entry| {
            let name = entry.file_name()?.to_str()?;
            let prefix = name.split('-').next()?;
            Some(prefix.to_string())
        })
        .filter_map(|prefix| prefix.parse::<u32>().ok())
        .max()
        .unwrap_or(0);

    let lesson_dir_name = format!("{:0>2}-{slug}", last_index + 1);
    let lesson_dir = directory.join(&lesson_dir_name);
    tokio::fs::create_dir(&lesson_dir).await.into_diagnostic()?;

    let manifest_path = lesson_dir.join("lesson.yaml");
    let metadata = LessonMetadata { name, slug };

    let config = init_config().await?;
    let server_base = Url::parse(&config.base_url).into_diagnostic()?;
    let serialized_manifest = serialize_with_schema(metadata.clone(), "lesson", server_base)
        .map_err(|_| miette!("Failed to serialize manifest"))?;

    fs::write(manifest_path, serialized_manifest)
        .await
        .into_diagnostic()?;

    println!(
        "Created {}",
        format!("{}/lesson.yaml", lesson_dir_name).bold().green()
    );

    let readme_path = lesson_dir.join("README.md");
    fs::write(&readme_path, format!("# {}\n\n", metadata.name))
        .await
        .into_diagnostic()?;

    println!(
        "Created {}",
        format!("{}/README.md", lesson_dir_name).bold().green()
    );

    Ok(())
}
