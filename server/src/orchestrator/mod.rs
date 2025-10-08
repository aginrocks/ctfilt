mod lifecycle;
mod resources;
mod types;
mod watcher;

use exterminator::{Extermination, Exterminator};
use headscale::apis::configuration::Configuration;
use kube::Client;
use std::sync::Arc;

use crate::{database::DatabaseStore, utils::FlagGenerator};

pub use lifecycle::*;
pub use types::*;
pub use watcher::*;

pub struct ChallengeOrchestrator {
    pub kube: Client,
    pub flags: Arc<FlagGenerator>,
    pub headscale_config: Arc<Configuration>,
    pub headscale_public_url: String,
    pub exterminator: Exterminator,
    pub watcher: PodWatcher,
}

impl ChallengeOrchestrator {
    pub fn new(
        kube: Client,
        flags: Arc<FlagGenerator>,
        headscale_config: Arc<Configuration>,
        headscale_public_url: String,
        extermination: Extermination,
        db_store: DatabaseStore,
    ) -> Self {
        Self {
            kube: kube.clone(),
            flags,
            headscale_config,
            headscale_public_url,
            exterminator: Exterminator::new(kube.clone(), extermination.clone()),
            watcher: PodWatcher::new(kube, extermination, db_store),
        }
    }
}
