use color_eyre::eyre::{Context, eyre};
use futures::TryStreamExt;
use manifests::LessonMetadata;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::{
    axum_error::{AxumError, AxumResult},
    mongo_id::object_id_as_string_required,
};

database_object!(Lesson {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]

    id: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    course: ObjectId,

    order: i32,

    #[serde(flatten)]
    metadata: LessonMetadata,

    content: String,

    attachments: Vec<String>,
});

#[derive(Clone)]
pub struct LessonStore {
    collection: Collection<Lesson>,
    partial_collection: Collection<PartialLesson>,
}

impl LessonStore {
    pub fn new(database: &Database) -> Self {
        const COLLECTION: &str = "courses";

        let collection = database.collection::<Lesson>(COLLECTION);
        let partial_collection = database.collection::<PartialLesson>(COLLECTION);
        Self {
            collection,
            partial_collection,
        }
    }

    pub async fn get_all(&self) -> AxumResult<Vec<Lesson>> {
        let cursor = self
            .collection
            .find(doc! {})
            .await
            .wrap_err("Failed to fetch lessons")?;

        let lessons = cursor.try_collect().await?;
        Ok(lessons)
    }

    pub async fn get_by_course_id(&self, course_id: ObjectId) -> AxumResult<Vec<Lesson>> {
        let cursor = self
            .collection
            .find(doc! { "course": course_id })
            .sort(doc! { "order": 1 })
            .await
            .wrap_err("Failed to fetch lessons")?;

        let lessons = cursor.try_collect().await?;
        Ok(lessons)
    }

    pub async fn get_by_slug(&self, course_id: ObjectId, slug: &str) -> AxumResult<Lesson> {
        let lesson = self
            .collection
            .find_one(doc! { "slug": slug, "course": course_id })
            .await
            .wrap_err("Failed to fetch lesson")?
            .ok_or_else(|| AxumError::not_found(eyre!("Lesson not found")))?;

        Ok(lesson)
    }
}
