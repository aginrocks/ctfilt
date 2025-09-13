use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg(feature = "validator")]
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct LessonMetadata {
    /// A short name for the lesson
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
    pub name: String,

    /// A list of challenge slugs included in the lesson
    pub challenges: Vec<String>,
}
