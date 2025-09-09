mod lifecycle;

use kube::Client;
use std::sync::Arc;

use crate::utils::FlagGenerator;

pub use lifecycle::*;

pub struct ChallengeOrchestrator {
    pub kube: Arc<Client>,
    pub flags: Arc<FlagGenerator>,
}

impl ChallengeOrchestrator {
    pub fn new(kube: Arc<Client>, flags: Arc<FlagGenerator>) -> Self {
        Self { kube, flags }
    }
}
