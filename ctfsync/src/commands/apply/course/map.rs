use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use api_client::{
    apis::course_api,
    models::{LessonMetadata, UpdateLessonRequest},
};
use dashmap::DashMap;
use miette::{IntoDiagnostic, Result};

use crate::{
    api::init_api_config,
    commands::apply::course::diff::{read_manifest, read_readme},
    errors::MissingLessonFiles,
    utils::read_slug,
};

pub struct LessonsMap {
    pub cache: Arc<DashMap<PathBuf, Entry>>,
    pub course_slug: String,
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub manifest: Option<LessonMetadata>,
    pub content: Option<String>,
    pub sent: bool,
}

impl LessonsMap {
    pub fn new(course_slug: String) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            course_slug,
        }
    }

    pub fn insert_manifest(&self, path: PathBuf, manifest: LessonMetadata) {
        self.cache.insert(
            path,
            Entry {
                manifest: Some(manifest),
                content: None,
                sent: false,
            },
        );
    }

    pub fn insert_content(&self, path: PathBuf, content: String) {
        self.cache.insert(
            path,
            Entry {
                manifest: None,
                content: Some(content),
                sent: false,
            },
        );
    }

    pub async fn send(&self, path: &Path, entry: Entry) -> Result<bool> {
        if entry.sent {
            return Ok(false);
        }

        let (order, slug) = read_slug(path)?;

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

        drop(entry);

        Ok(true)
    }

    // TODO: Use Semaphore to allow for limited concurrency
    pub async fn send_all(&self) -> Result<()> {
        for item in self.cache.iter() {
            let (path, entry) = item.pair();
            if !entry.sent {
                self.send(path, entry.clone()).await?;
            }
        }

        Ok(())
    }
}
