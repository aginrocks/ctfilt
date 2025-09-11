use serde::{Deserialize, Serialize};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ChallengeFlagMeta {
    Static {
        /// The static flag for the challenge
        flag: String,
    },
    DynamicMount {
        /// Where the flag should be mounted inside the container
        mount_path: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ChallengeFlag {
    pub points: i32,

    /// A short description where the flag can be found.
    /// Can be revealed in courses.
    /// Visible only after solving the challenge in contests.
    pub description: Option<String>,

    #[serde(flatten)]
    pub meta: ChallengeFlagMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ChallengeMetadata {
    /// A short unique name for the challenge
    pub name: String,

    /// A URL-friendly unique identifier for the challenge
    pub slug: String,

    /// Markdown description of the challenge
    pub description: String,

    #[serde(flatten)]
    pub spec: ChallengeSpec,

    pub flags: Vec<ChallengeFlag>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChallengeSpec {
    /// A fully static challenge with one answer
    Static {},

    /// A challenge with a custom validator
    Dynamic {},

    /// A challenge that requires VPN use and is created per user
    Container {
        /// The container image for the container challenge
        image: String,
    },
}
