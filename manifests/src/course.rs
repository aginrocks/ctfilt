use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

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

/// # Generics:
///
/// - `Ref`: The type used to reference lessons and challenges
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(untagged)]
pub enum CourseItem<Ref> {
    Lesson {
        /// Lesson slug referencing a lesson in the same repository
        lesson: Ref,
    },
    Challenge {
        /// Challenge slug referencing any challenge on the server
        challenge: Ref,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum CourseItemType {
    Lesson,
    Challenge,
}

/// Metadata for a course
///
/// # Generics:
///
/// - `Item`: The type used to store individual course item
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct CourseMetadata<Item> {
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

    /// Items included in the course (order matters)
    pub items: Vec<Item>,
}
