use color_eyre::eyre::Result;
use futures::TryStreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    Api, Client,
    api::ListParams,
    runtime::{
        self,
        watcher::{Config, Event},
    },
};
use std::pin::pin;
use tracing::info;

use crate::settings::Settings;

pub async fn watch_pods(client: Client, settings: &Settings) -> Result<()> {
    let pods: Api<Pod> = Api::default_namespaced(client);

    let selector = format!("{}/enable=true", settings.labels.prefix);
    let params = ListParams::default().labels(&selector);

    info!("Listing managed pods that were created before startup");

    let managed_pods = pods.list(&params).await?;
    for pod in managed_pods {
        handle_pod(pod).await?;
    }

    info!("Starting watcher");

    let watcher = runtime::watcher(pods, Config::default().labels(&selector));
    let mut watcher = pin!(watcher);

    while let Some(event) = watcher.try_next().await? {
        if let Event::Apply(pod) = event {
            handle_pod(pod).await?;
        }
    }

    Ok(())
}

pub async fn handle_pod(pod: Pod) -> Result<()> {
    info!(name = pod.metadata.name, "Handling pod");
    Ok(())
}
