use std::sync::Arc;

use headscale::apis::configuration::Configuration;
use mongodb::Database;

use crate::{
    database::DatabaseStore, orchestrator::ChallengeOrchestrator, settings::Settings,
    utils::FlagGenerator, vpn::Vpn,
};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub store: DatabaseStore,
    pub settings: Arc<Settings>,
    pub kube: kube::Client,
    pub flags: Arc<FlagGenerator>,
    pub orchestrator: Arc<ChallengeOrchestrator>,
    pub headscale_config: Arc<Configuration>,
    pub fred: fred::prelude::Pool,
    pub vpn: Arc<dyn Vpn>,
}
