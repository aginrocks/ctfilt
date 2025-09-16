use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

use strum::{Display, EnumIter, EnumString};
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg(feature = "validator")]
use {crate::validators::slug_validator, validator::Validate};

#[cfg(feature = "clap")]
use clap::ValueEnum;

#[derive(Debug, Serialize, Deserialize, Clone, Default, EnumIter, EnumString, Display)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "clap", derive(ValueEnum), clap(rename_all = "lower"))]
#[serde(rename_all = "lowercase")]
pub enum CourseDifficulty {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
    pub objectives: Option<Vec<String>>,

    /// A list of course slugs that are prerequisites for this course
    pub prerequisites: Option<Vec<String>>,

    /// The difficulty level of the course
    pub difficulty: CourseDifficulty,
}
