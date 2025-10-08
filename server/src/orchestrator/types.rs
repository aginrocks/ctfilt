use chrono::DateTime;
use derive_builder::Builder;
use k8s_openapi::chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

use crate::mongo_id::object_id_as_string_required;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default, TS, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChallengeStatus {
    #[default]
    Starting,
    Running,
    Stopping,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Builder, Default, TS, PartialEq, Eq)]
pub struct RunningChallenge {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    #[ts(as = "String")]
    pub id: ObjectId,

    pub status: ChallengeStatus,

    pub hostname: Option<String>,

    pub ip: Option<String>,

    pub expires_at: DateTime<Utc>,

    #[serde(flatten)]
    pub metadata: PodCachedMetadata,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Default, TS, PartialEq, Eq)]
pub struct PodCachedMetadata {
    pub name: String,

    pub slug: String,
}
