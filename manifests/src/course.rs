use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg(feature = "validator")]
use {crate::validators::slug_validator, validator::Validate};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CourseDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct CourseMetadata {
    /// A short unique name for the course
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
    pub name: String,

    /// A URL-friendly unique identifier for the course
    #[cfg_attr(
        feature = "validator",
        validate(custom(function = "slug_validator"), length(min = 1, max = 32))
    )]
    pub slug: String,

    /// Tags associated with the course
    pub tags: Option<Vec<String>>,

    /// A short description of the course
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128)))]
    pub description: String,

    /// A list of learning objectives for the course
    pub objectives: Vec<String>,

    /// A list of course slugs that are prerequisites for this course
    pub prerequisites: Option<Vec<String>>,

    /// The difficulty level of the course
    pub difficulty: CourseDifficulty,
}
