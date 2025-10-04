pub mod readme;

use clap::Subcommand;
use miette::Result;

#[derive(Subcommand, Debug, Clone)]
pub enum GenerateCommands {
    /// Generate a README
    Readme,
}

pub async fn handle_generate(cmd: GenerateCommands) -> Result<()> {
    match cmd {
        GenerateCommands::Readme => readme::run().await,
    }
}
