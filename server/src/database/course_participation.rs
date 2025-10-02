use color_eyre::eyre::{Context, eyre};
use manifests::CourseItemType;
use manifests::slug_validator;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use visible::StructFields;

use crate::{
    axum_error::{AxumError, AxumResult},
    mongo_id::{object_id_as_string, object_id_as_string_required},
};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, Validate)]
pub struct LastItem {
    pub r#type: CourseItemType,

    #[validate(custom(function = "slug_validator"), length(min = 1, max = 32))]
    pub slug: String,
}

database_object!(CourseParticipation {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    course: ObjectId,

    /// Slug of last viewed lesson or challenge
    last_item: Option<LastItem>,
});

#[derive(Clone)]
pub struct CourseParticipationStore {
    collection: Collection<CourseParticipation>,
    partial_collection: Collection<PartialCourseParticipation>,
}

impl CourseParticipationStore {
    pub fn new(database: &Database) -> Self {
        const COLLECTION: &str = "course_participations";

        let collection = database.collection::<CourseParticipation>(COLLECTION);
        let partial_collection = database.collection::<PartialCourseParticipation>(COLLECTION);
        Self {
            collection,
            partial_collection,
        }
    }

    pub async fn update(
        &self,
        user: ObjectId,
        course: ObjectId,
        last_item: LastItem,
    ) -> AxumResult<()> {
        self.partial_collection
            .find_one_and_replace(
                doc! { "user": user, "course": course },
                PartialCourseParticipation {
                    user,
                    course,
                    last_item: Some(last_item),
                },
            )
            .upsert(true)
            .await
            .wrap_err("Failed to update course participation")?;

        Ok(())
    }
}
