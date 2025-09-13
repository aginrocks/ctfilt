use color_eyre::eyre::Context;
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

use crate::{axum_error::AxumResult, mongo_id::object_id_as_string_required};

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

    r#ref: String,
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
}
