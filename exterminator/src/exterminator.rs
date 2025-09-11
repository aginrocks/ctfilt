use color_eyre::eyre::Result;
use kube::Client;

use crate::Extermination;

pub struct Exterminator {
    pub client: Client,
    pub settings: Extermination,
}

impl Exterminator {
    pub fn new(client: Client, settings: Extermination) -> Self {
        Self { client, settings }
    }

    pub async fn exterminate(&self, pod_name: &str) -> Result<()> {
        Ok(())
    }
}
