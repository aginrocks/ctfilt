use std::path::Path;

use tokio::fs;

pub enum RepoType {
    Course,
    Challenge,
    Unknown,
}

pub async fn detect_repo_type(repo_path: &Path) -> RepoType {
    let course_path = repo_path.join("course.yaml");
    let challenge_path = repo_path.join("challenge.yaml");

    if fs::metadata(&course_path).await.is_ok() {
        RepoType::Course
    } else if fs::metadata(&challenge_path).await.is_ok() {
        RepoType::Challenge
    } else {
        RepoType::Unknown
    }
}
