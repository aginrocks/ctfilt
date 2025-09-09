use std::{ops::Deref, sync::Arc};

use headscale::apis::configuration::Configuration;
use mongodb::Database;

use crate::{orchestrator::ChallengeOrchestrator, settings::Settings, utils::FlagGenerator};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Arc<Settings>,
    pub kube: kube::Client,
    pub flags: Arc<FlagGenerator>,
    pub orchestrator: Arc<ChallengeOrchestrator>,
    pub headscale_config: Arc<Configuration>,
}
