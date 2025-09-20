#[macro_use]
pub mod macros;

pub mod access_token;
pub mod challenge;
pub mod course;
pub mod init;
pub mod lesson;
pub mod user;

pub use access_token::*;
pub use challenge::*;
pub use course::*;
pub use init::*;
pub use lesson::*;
use mongodb::{Client, Database, bson::oid::ObjectId};
pub use user::*;

use crate::axum_error::AxumResult;

#[derive(Clone)]
pub struct DatabaseStore {
    pub database: Database,
    pub courses: CourseStore,
    pub lessons: LessonStore,
}

impl DatabaseStore {
    pub fn new(database: &Database) -> Self {
        Self {
            database: database.clone(),
            courses: CourseStore::new(database),
            lessons: LessonStore::new(database),
        }
    }

    pub async fn resolve_course_items(
        &self,
        course_slug: &str,
        items: Vec<manifests::CourseItem>,
    ) -> AxumResult<Vec<CourseItem>> {
        let mut result = Vec::new();

        for item in items {
            let resolved_item = match item {
                manifests::CourseItem::Lesson { lesson } => {
                    let lesson = self.lessons.get_by_slug(course_slug, &lesson).await?;
                    CourseItem::Lesson { lesson: lesson.id }
                }
                manifests::CourseItem::Challenge { challenge } => todo!(),
            };
            result.push(resolved_item);
        }

        Ok(result)
    }
}
