use color_eyre::eyre::Result;
use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};

use crate::settings::Settings;

pub async fn kill_pod(client: Client, settings: &Settings, pod_name: &str) -> Result<()> {
    let pods: Api<Pod> = Api::default_namespaced(client);

    Ok(())
}
