use std::sync::Arc;

use gitea_client::apis::configuration::Configuration;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub gitea: Arc<Configuration>,
}
