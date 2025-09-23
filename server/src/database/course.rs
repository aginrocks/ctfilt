use color_eyre::eyre::{Context, eyre};
use futures::TryStreamExt;
use manifests::{CourseItem, CourseMetadata};
use mongo_utils::{JoinPipeline, JoinPipelineBuilder};
use mongodb::{
    Collection, Database,
    bson::{self, doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{ChallengeStore, LessonStore},
    mongo_id::object_id_as_string_required,
};

database_object!(Course {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    #[schema(value_type = CourseMetadata<CourseItem<String>>)]
    metadata: CourseMetadata<CourseItem<ObjectId>>,

    r#ref: String,
});

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DetailedCourse {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    #[schema(value_type = CourseMetadata<CourseItem<String>>)]
    metadata: CourseMetadata<DetailedCourseItem>,

    r#ref: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DetailedCourseItem {
    Lesson(DetailedCourseLesson),
    Challenge(DetailedCourseChallenge),
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, JoinPipeline)]
#[mongo_utils(collection = "lessons")]
pub struct DetailedCourseLesson {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub id: ObjectId,

    pub name: String,

    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, JoinPipeline)]
#[mongo_utils(collection = "challenges")]
pub struct DetailedCourseChallenge {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    pub id: ObjectId,

    pub name: String,

    pub slug: String,
    // TODO: Add more challenge-specific fields
}

impl DetailedCourseLesson {
    pub fn id(&self) -> ObjectId {
        self.id
    }
}

#[derive(Clone)]
pub struct CourseStore {
    collection: Collection<Course>,
    partial_collection: Collection<PartialCourse>,
    database: Database,
}

impl CourseStore {
    pub fn new(database: &Database) -> Self {
        const COLLECTION: &str = "courses";

        let collection = database.collection::<Course>(COLLECTION);
        let partial_collection = database.collection::<PartialCourse>(COLLECTION);
        Self {
            collection,
            partial_collection,
            database: database.clone(),
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

    pub async fn get_by_slug_full(&self, slug: &str) -> AxumResult<DetailedCourse> {
        let course = self.get_by_slug(slug).await?;
        let items = course.metadata.items;

        let lesson_store = LessonStore::new(&self.database);
        let lesson_ids = items
            .iter()
            .cloned()
            .filter_map(|item| match item {
                CourseItem::Lesson { lesson } => Some(lesson),
                _ => None,
            })
            .collect::<Vec<_>>();

        // TODO: Drop unused fields
        let lessons = lesson_store.get_many(lesson_ids).await?;

        let challenge_store = ChallengeStore::new(&self.database);
        let challenge_ids = items
            .iter()
            .cloned()
            .filter_map(|item| match item {
                CourseItem::Challenge { challenge } => Some(challenge),
                _ => None,
            })
            .collect::<Vec<_>>();

        let challenges = challenge_store.get_many(challenge_ids).await?;

        // Ok(course)
        todo!()
    }
}
