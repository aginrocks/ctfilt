use manifests::ChallengeMetadata;
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChallengeParent {
    Course {
        #[serde(with = "object_id_as_string_required")]
        #[schema(value_type = String)]
        course_id: ObjectId,

        #[serde(with = "object_id_as_string_required")]
        #[schema(value_type = String)]
        lesson_id: ObjectId,
    },
    Contest {
        #[serde(with = "object_id_as_string_required")]
        #[schema(value_type = String)]
        contest_id: ObjectId,
    },
}

database_object!(Challenge {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    metadata: ChallengeMetadata,

    /// The parent of the challenge, either a course or a contest
    /// It's not included in the metadata because its course or contest
    /// will be inferred from the context of the file
    parent: ChallengeParent,
});
