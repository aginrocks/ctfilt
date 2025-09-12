use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Config saving failed")]
#[diagnostic(code(config::save_fail), help("Unable to save configuration file"))]
pub struct ConfigSavingFailed;

#[derive(Error, Debug, Diagnostic)]
#[error("You are not logged in")]
#[diagnostic(
    code(auth::not_logged_in),
    help("Use the 'ctfsync auth login' command to log in")
)]
pub struct NotLoggedIn;
