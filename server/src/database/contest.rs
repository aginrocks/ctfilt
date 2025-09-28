use manifests::{ContestBatch, ContestMetadata};
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;

database_object!(Contest {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    metadata: ContestMetadata,

    #[schema(value_type = Vec<ContestBatch<String>>)]
    batches: Vec<ContestBatch<ObjectId>>,

    /// Commit hash of the repository the contest was imported from
    r#ref: String,
});
