use std::{collections::HashMap, path::PathBuf};

use api_client::{
    apis::course_api,
    models::{LessonMetadata, UpdateLessonRequest},
};
use miette::{IntoDiagnostic, Result};

use crate::{
    api::init_api_config,
    commands::apply::course::diff::{read_manifest, read_readme},
    errors::MissingLessonFiles,
    utils::read_slug,
};

pub struct LessonsMap {
    pub cache: HashMap<PathBuf, Entry>,
    pub course_slug: String,
}

pub struct Entry {
    pub manifest: Option<LessonMetadata>,
    pub content: Option<String>,
    pub sent: bool,
}

// TODO: make thread-safe
impl LessonsMap {
    pub fn new(course_slug: String) -> Self {
        Self {
            cache: HashMap::new(),
            course_slug,
        }
    }

    pub fn insert_manifest(&mut self, path: PathBuf, manifest: LessonMetadata) {
        self.cache.insert(
            path,
            Entry {
                manifest: Some(manifest),
                content: None,
                sent: false,
            },
        );
    }

    pub fn insert_content(&mut self, path: PathBuf, content: String) {
        self.cache.insert(
            path,
            Entry {
                manifest: None,
                content: Some(content),
                sent: false,
            },
        );
    }

    pub async fn send(&mut self, path: PathBuf) -> Result<bool> {
        let entry = self.cache.get_mut(&path).ok_or(MissingLessonFiles)?;
        if entry.sent {
            return Ok(false);
        }

        let (order, slug) = read_slug(&path)?;

        let metadata = match entry.manifest {
            Some(ref manifest) => manifest.clone(),
            None => {
                let manifest_path = path.join("lesson.yaml");
                read_manifest(&manifest_path).await?
            }
        };

        let content = match entry.content {
            Some(ref content) => content.clone(),
            None => {
                let readme_path = path.join("README.md");
                read_readme(&readme_path).await?
            }
        };

        let config = init_api_config().await?;

        let body = UpdateLessonRequest {
            metadata: Box::new(metadata.clone()),
            content,
            order,
        };
        course_api::update_course_lesson(config, &self.course_slug, &metadata.slug, body)
            .await
            .into_diagnostic()?;

        entry.sent = true;

        Ok(true)
    }
}
