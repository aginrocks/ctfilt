use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ChallengeType {
    /// A fully static challenge with one answer
    Static,

    /// A challenge with a custom validator
    Dynamic,

    /// A challenge that requires VPN use and is created per user
    Container,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChallengeFlagMeta {
    Static {
        /// The static flag for the challenge
        flag: String,
    },
    Dynamic {
        /// Where the flag should be mounted inside the container
        mount_path: String,
    },
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeFlag {
    points: i32,
    
    /// A short description where the flag can be found.
    /// Can be revealed in courses.
    /// Visible only after solving the challenge in contests.
    description: Option<String>,

    #[serde(flatten)]
    meta: ChallengeFlagMeta,
}

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

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeMetadata {
    /// A short unique name for the challenge
    name: String,

    /// A URL-friendly unique identifier for the challenge
    slug: String,

    /// Markdown description of the challenge
    description: String,

    #[serde(flatten)]
    spec: ChallengeSpec,

    flags: Vec<ChallengeFlag>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChallengeSpec {
    Static {},
    Dynamic {},
    Container {
        /// The container image for the container challenge
        image: String,
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
