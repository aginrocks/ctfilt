mod lesson;

use clap::Subcommand;
use miette::Result;

#[derive(Subcommand, Debug, Clone)]
pub enum NewCommands {
    /// Create a new lesson inside a course
    #[command(alias = "le")]
    Lesson {
        #[command(flatten)]
        args: lesson::InitLessonArgs,
    },
}

pub async fn handle_new(cmd: NewCommands) -> Result<()> {
    match cmd {
        NewCommands::Lesson { args } => lesson::run(args).await,
    }
}
