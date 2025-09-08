use std::{ops::Deref, sync::Arc};

use mongodb::Database;

use crate::{settings::Settings, utils::FlagGenerator};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub settings: Arc<Settings>,
    pub kube: Arc<kube::Client>,
    pub flags: Arc<FlagGenerator>,
}
