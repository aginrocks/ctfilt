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
// Had to use untagged enums for JsonSchema compatibility
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
#[serde(untagged)]
pub enum ChallengeFlagMeta {
    Static(ChallengeFlagStatic),
    Dynamic(ChallengeFlagDynamic),
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeFlag {
    points: i32,

    #[serde(flatten)]
    meta: ChallengeFlagMeta,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub enum StaticFlag {
    /// A static flag (the same for all users)
    #[serde(rename = "static")]
    Value,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeFlagStatic {
    r#type: StaticFlag,

    /// The static flag for the challenge
    flag: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub enum DynamicFlag {
    /// A flag that is different for
    #[serde(rename = "dynamic")]
    Value,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeFlagDynamic {
    r#type: DynamicFlag,

    /// Where the flag should be mounted inside the container
    mount_path: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, JsonSchema)]
pub struct ChallengeMetadata {
    /// A short unique name for the challenge
    name: String,

    /// A URL-friendly unique identifier for the challenge
    slug: String,

    /// Markdown description of the challenge
    description: String,

    r#type: ChallengeType,

    flags: Vec<ChallengeFlag>,
}

database_object!(Challenge {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    metadata: ChallengeMetadata,
});
