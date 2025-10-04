pub mod challenge;
pub mod course;

use clap::Subcommand;
use miette::Result;

#[derive(Subcommand, Debug, Clone)]
pub enum InitCommands {
    /// Initialize a course
    #[command(alias = "co")]
    Course {
        #[command(flatten)]
        args: course::InitCourseArgs,
    },
    /// Initialize a challenge
    #[command(alias = "ch")]
    Challenge {
        #[command(flatten)]
        args: challenge::InitChallengeArgs,
    },
}

pub async fn handle_init(cmd: InitCommands) -> Result<()> {
    match cmd {
        InitCommands::Course { args } => course::run(args).await,
        InitCommands::Challenge { args } => challenge::run(args).await,
    }
}
