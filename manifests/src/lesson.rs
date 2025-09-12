use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct LessonMetadata {
    /// A short name for the lesson
    pub name: String,

    /// A list of challenge slugs included in the lesson
    pub challenges: Vec<String>,
}
