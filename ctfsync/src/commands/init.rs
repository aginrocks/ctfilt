pub mod course;

use clap::Subcommand;
use miette::Result;

#[derive(Subcommand, Debug, Clone)]
pub enum InitCommands {
    /// Initialize a course
    Course {
        #[command(flatten)]
        args: course::InitCourseArgs,
    },
    /// Initialize a challenge
    Challenge,
}

pub async fn handle_init(cmd: InitCommands) -> Result<()> {
    match cmd {
        InitCommands::Course { args } => course::run(args).await,
        _ => todo!(),
    }
}
