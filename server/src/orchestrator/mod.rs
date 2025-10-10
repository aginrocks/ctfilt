mod lifecycle;
mod resources;
mod types;
mod watcher;

use exterminator::{Extermination, Exterminator};
use headscale::apis::configuration::Configuration;
use kube::Client;
use std::sync::Arc;

use crate::{database::DatabaseStore, utils::FlagGenerator, vpn::Vpn};

pub use lifecycle::*;
pub use types::*;
pub use watcher::*;

pub struct ChallengeOrchestrator {
    pub kube: Client,
    pub flags: Arc<FlagGenerator>,
    pub exterminator: Exterminator,
    pub watcher: PodWatcher,
    pub vpn: Arc<dyn Vpn>,
}

impl ChallengeOrchestrator {
    pub fn new(
        kube: Client,
        flags: Arc<FlagGenerator>,
        extermination: Extermination,
        db_store: DatabaseStore,
        vpn: Arc<dyn Vpn>,
    ) -> Self {
        Self {
            kube: kube.clone(),
            flags,
            exterminator: Exterminator::new(kube.clone(), extermination.clone()),
            watcher: PodWatcher::new(kube, extermination, db_store),
            vpn,
        }
    }
}
