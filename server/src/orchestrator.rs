mod lifecycle;
mod resources;
mod types;

use headscale::apis::configuration::Configuration;
use kube::Client;
use std::sync::Arc;

use crate::utils::FlagGenerator;

pub use lifecycle::*;
pub use types::*;

pub struct ChallengeOrchestrator {
    pub kube: Arc<Client>,
    pub flags: Arc<FlagGenerator>,
    pub headscale_config: Arc<Configuration>,
}

impl ChallengeOrchestrator {
    pub fn new(
        kube: Arc<Client>,
        flags: Arc<FlagGenerator>,
        headscale_config: Arc<Configuration>,
    ) -> Self {
        Self {
            kube,
            flags,
            headscale_config,
        }
    }
}
