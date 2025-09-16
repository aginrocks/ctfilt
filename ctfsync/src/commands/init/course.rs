use clap::Parser;
use inquire::Text;
use manifests::{CourseDifficulty, to_slug};
use miette::{IntoDiagnostic, Result};
use strum::IntoEnumIterator;
use tokio::task;

#[derive(Debug, Clone, Parser)]
pub struct InitCourseArgs {
    slug: Option<String>,

    #[arg(short = 'n', long)]
    name: Option<String>,

    #[arg(short = 'd', long)]
    description: Option<String>,

    #[arg(short = 'D', long, value_enum)]
    difficulty: Option<CourseDifficulty>,
}

pub async fn run(args: InitCourseArgs) -> Result<()> {
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

    let options = CourseDifficulty::iter().collect::<Vec<_>>();

    // let diffic

    dbg!(options);
    Ok(())
}
