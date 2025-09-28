use color_eyre::eyre::{Context, eyre};
use futures::TryStreamExt;
use manifests::CourseMetadata;
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

database_object!(Course {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    #[schema(value_type = CourseMetadata<String>)]
    metadata: CourseMetadata<ObjectId>,

    /// Commit hash of the repository the course was imported from
    r#ref: String,
});

#[derive(Clone)]
pub struct CourseStore {
    collection: Collection<Course>,
    partial_collection: Collection<PartialCourse>,
}

impl CourseStore {
    pub fn new(database: &Database) -> Self {
        const COLLECTION: &str = "courses";

        let collection = database.collection::<Course>(COLLECTION);
        let partial_collection = database.collection::<PartialCourse>(COLLECTION);
        Self {
            collection,
            partial_collection,
        }
    }

    pub async fn get_all(&self) -> AxumResult<Vec<Course>> {
        let cursor = self
            .collection
            .find(doc! {})
            .await
            .wrap_err("Failed to fetch courses")?;

        let courses = cursor.try_collect().await?;
        Ok(courses)
    }

    pub async fn get_by_slug(&self, slug: &str) -> AxumResult<Course> {
        let course = self
            .collection
            .find_one(doc! { "slug": slug })
            .await
            .wrap_err("Failed to fetch course")?
            .ok_or_else(|| AxumError::not_found(eyre!("Course not found")))?;

        Ok(course)
    }
}
