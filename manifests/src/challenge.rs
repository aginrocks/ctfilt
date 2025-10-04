use serde::{Deserialize, Serialize};

use crate::utils::{deserialize_octal_option, serialize_octal_option};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg(feature = "validator")]
use {crate::validators::slug_validator, validator::Validate};

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

        /// File permissions for the flag file, in octal format (e.g. 644, defaults to 600)
        #[serde(
            default,
            serialize_with = "serialize_octal_option",
            deserialize_with = "deserialize_octal_option"
        )]
        #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
        #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>))]
        permissions: Option<i32>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct ChallengeFlag {
    /// Slug that will allow this flag to be referenced in contests
    #[cfg_attr(
        feature = "validator",
        validate(custom(function = "slug_validator"), length(min = 1, max = 32))
    )]
    pub slug: String,

    /// A short description where the flag can be found.
    /// Visible only after solving the challenge.
    pub description: Option<String>,

    // #[serde(flatten)]
    pub spec: ChallengeFlagMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "validator", derive(Validate))]
pub struct ChallengeMetadata {
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

    /// Markdown description of the challenge
    pub details: String,

    // #[serde(flatten)]
    pub spec: ChallengeSpec,

    pub flags: Vec<ChallengeFlag>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum PublicFlag {
    Submitted {
        /// A short description where the flag can be found.
        /// Visible only after solving the challenge.
        description: Option<String>,

        /// Current points value of this flag
        points: i32,
    },
    NotSubmitted {
        /// Current points value of this flag
        points: i32,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct PublicChallengeMetadata {
    /// A short unique name for the challenge
    pub name: String,

    /// A URL-friendly unique identifier for the challenge
    pub slug: String,

    /// A short description, Markdown not supported
    pub description: String,

    /// Markdown description of the challenge
    pub details: String,

    pub flags: Vec<PublicFlag>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ChallengeContainer {
    pub name: String,
    pub args: Option<Vec<String>>,
    pub command: Option<Vec<String>>,
    pub image: String,
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
        /// Containers that should be created in the challenge Pod
        containers: Vec<ChallengeContainer>,
    },
}

impl Default for ChallengeSpec {
    fn default() -> Self {
        Self::Static {}
    }
}
