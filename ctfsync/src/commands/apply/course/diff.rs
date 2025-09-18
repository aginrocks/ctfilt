use std::path::{Path, PathBuf};

use api_client::models::LessonMetadata;
use gix::diff::tree_with_rewrites::Change;
use miette::{IntoDiagnostic, Result};
use owo_colors::OwoColorize;
use tokio::fs;

use crate::{commands::apply::course::map::LessonsMap, errors::OutOfScopeFile};

pub async fn apply_diff(diff: Vec<Change>, directory: &Path, course_slug: String) -> Result<()> {
    let mut lessons_queue = LessonsMap::new(course_slug);

    for change in diff {
        let logged_change = match change {
            Change::Addition { .. } => change.location().green().to_string(),
            Change::Deletion { .. } => change.location().red().to_string(),
            Change::Modification { .. } | Change::Rewrite { .. } => {
                change.location().yellow().to_string()
            }
        };
        println!("Applying {}", logged_change.bold());

        // TODO: Use Semaphore to allow for limited concurrency
        match change {
            Change::Addition { .. } | Change::Modification { .. } => {
                let path = directory.join(change.location().to_string());
                let file_type = detect_file(&path).await?;
                let parent = path.parent().ok_or(OutOfScopeFile)?;

                match file_type {
                    FileDetectionResult::LessonManifest(content) => {
                        lessons_queue.insert_manifest(parent.to_path_buf(), content);
                    }
                    FileDetectionResult::Readme(content) => {
                        lessons_queue.insert_content(parent.to_path_buf(), content);
                    }
                    _ => todo!(),
                }
            }
            Change::Deletion { .. } | Change::Rewrite { .. } => {
                todo!();
            }
        }
    }

    Ok(())
}

pub enum FileDetectionResult {
    Attachment,
    LessonManifest(LessonMetadata),
    Readme(String),
}

pub async fn detect_file(path: &PathBuf) -> Result<FileDetectionResult> {
    let filename = path.file_name().ok_or(OutOfScopeFile)?;
    if filename == "README.md" {
        Ok(FileDetectionResult::Readme(read_readme(path).await?))
    } else if filename == "lesson.yaml" {
        Ok(FileDetectionResult::LessonManifest(
            read_manifest(path).await?,
        ))
    } else {
        Ok(FileDetectionResult::Attachment)
    }
}

pub async fn read_readme(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path).await.into_diagnostic()?;
    Ok(content)
}

pub async fn read_manifest(path: &PathBuf) -> Result<LessonMetadata> {
    let content = fs::read_to_string(path).await.into_diagnostic()?;
    let manifest = serde_yaml::from_str(&content).into_diagnostic()?;
    Ok(manifest)
}
