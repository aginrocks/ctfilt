use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;

database_object!(Solve {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    challenge: ObjectId,

    /// Flag referenced by its slug
    flag: Option<String>,

    submitted_at: DateTime<Utc>,

    /// Raw text submitted by the user
    raw_submission: String,

    /// Whether the submission was correct
    correct: bool,
});
