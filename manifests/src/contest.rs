use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg(feature = "validator")]
use {crate::validators::slug_validator, validator::Validate};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ContestJoinPolicy {
    /// Anyone on the platform can join the contest
    #[default]
    Open,
    /// Only people with a join code can join the contest
    JoinCode { code: String },
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct ContestMetadata {
    /// A short name for the challenge
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
    pub name: String,

    /// A URL-friendly unique identifier for the challenge
    #[cfg_attr(
        feature = "validator",
        validate(custom(function = "slug_validator"), length(min = 1, max = 32))
    )]
    pub slug: String,

    /// A short description, Markdown not supported
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128)))]
    pub description: String,

    /// Markdown description of the contest
    pub details: String,

    /// Rules for the contest
    pub rules: Option<String>,

    /// Policy for joining the contest
    pub join_policy: ContestJoinPolicy,

    /// Pre-defined difficulty levels for challenges and their scoring rules
    pub difficulty_levels: HashMap<String, ScoringRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct ContestBatch<Ref> {
    /// Batch start date in RFC 3339 (empty will mean the batch is available immediately)
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub start: Option<DateTime<Utc>>,

    /// Batch end date in RFC 3339 (empty will mean the batch never ends)
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub end: Option<DateTime<Utc>>,

    /// A short name for the batch (e.g. "Day 1")
    #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
    pub name: String,

    /// A URL-friendly unique identifier (in the scope of this contest) for the batch
    #[cfg_attr(
        feature = "validator",
        validate(custom(function = "slug_validator"), length(min = 1, max = 32))
    )]
    pub slug: String,

    /// Markdown description of the batch
    pub details: String,

    /// Bindings of individual challenges to this batch
    pub challenges: Vec<ChallengeBinding<Ref>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "scoring", rename_all = "kebab-case")]
pub enum ScoringRule {
    Static {
        points: i32,
    },
    Dynamic {
        function: ScoringFunction,
        initial: i32,
        decay: i32,
        minimum: i32,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "kebab-case")]
pub enum ScoringFunction {
    /// Calculated as `Initial - (Decay * SolveCount)`
    Linear,
    /// Calculated as `(((Minimum - Initial) / (Decay^2)) * (SolveCount^2)) + Initial`
    Logarithmic,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct ChallengeBinding<Ref> {
    /// Challenge slug referencing any challenge on the server
    pub challenge: Ref,

    pub flags: Vec<FlagBinding>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct FlagBinding {
    /// Slug that will allow this flag to be referenced in contests
    #[cfg_attr(
        feature = "validator",
        validate(custom(function = "slug_validator"), length(min = 1, max = 32))
    )]
    pub slug: String,

    /// Difficulty level for this flag (must match one of the difficulty levels defined in the chalenge manifest)
    #[cfg_attr(feature = "validator", validate(length(min = 1)))]
    pub difficulty_level: String,
}
