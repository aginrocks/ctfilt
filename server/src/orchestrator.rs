mod lifecycle;
mod resources;
mod types;

use exterminator::Exterminator;
use headscale::apis::configuration::Configuration;
use kube::Client;
use std::sync::Arc;

use crate::utils::FlagGenerator;

pub use lifecycle::*;
pub use types::*;

pub struct ChallengeOrchestrator {
    pub kube: Client,
    pub flags: Arc<FlagGenerator>,
    pub headscale_config: Arc<Configuration>,
    pub headscale_public_url: String,
    pub exterminator: Exterminator,
}

impl ChallengeOrchestrator {
    pub fn new(
        kube: Client,
        flags: Arc<FlagGenerator>,
        headscale_config: Arc<Configuration>,
        headscale_public_url: String,
        exterminator: Exterminator,
    ) -> Self {
        Self {
            kube,
            flags,
            headscale_config,
            headscale_public_url,
            exterminator,
        }
    }
}
