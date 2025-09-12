pub mod login;
pub mod logout;

use clap::Subcommand;
use miette::Result;

use crate::Cli;

#[derive(Subcommand, Debug, Clone)]
pub enum AuthCommands {
    Login {
        #[command(flatten)]
        args: login::LoginArgs,
    },
    Logout,
}

pub async fn handle_auth(cli: &Cli, cmd: AuthCommands) -> Result<()> {
    match cmd {
        AuthCommands::Login { args } => login::run(args).await,
        // AuthCommands::Logout => logout::run(cli).await,
        _ => todo!(),
    }
}
