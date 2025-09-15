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

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to fetch token information")]
#[diagnostic(
    code(auth::invalid_token),
    help("Ensure that the base URL and token is valid")
)]
pub struct InvalidToken;

#[derive(Error, Debug, Diagnostic)]
#[error("Failed to read Git data")]
#[diagnostic(
    code(git::no_repo),
    help("Ensure that you are inside a Git repository")
)]
pub struct NoGitRepo;

#[derive(Error, Debug, Diagnostic)]
#[error("Git working directory could not be found")]
#[diagnostic(code(git::no_workidr))]
pub struct NoGitWorkdir;

#[derive(Error, Debug, Diagnostic)]
#[error("Manifest could not be found")]
#[diagnostic(
    code(repo::no_manifest),
    help("Ensure that 'course.yaml' or 'challenge.yaml' exists in the repository root")
)]
pub struct NoManifest;

#[derive(Error, Debug, Diagnostic)]
#[error("Your repository has uncommitted changes")]
#[diagnostic(
    code(repo::dirty),
    help("Commit or stash your changes before applying configuration")
)]
pub struct DityWorktree;
