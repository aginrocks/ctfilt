use std::{ops::Deref, sync::Arc};

use mongodb::Database;

use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Arc<Settings>,
    pub kube: Arc<kube::Client>,
}
