use color_eyre::eyre::Result;
use kube::{Client, Config};

use crate::settings::{KubernetesMode, Settings};

pub async fn init_kubernetes(settings: &Settings) -> Result<Client> {
    let config = match settings.kubernetes.mode {
        KubernetesMode::InCluster => Config::incluster_env()?,
        KubernetesMode::Kubeconfig => Config::from_kubeconfig(&Default::default()).await?,
    };

    let client = Client::try_from(config)?;
    Ok(client)
}
