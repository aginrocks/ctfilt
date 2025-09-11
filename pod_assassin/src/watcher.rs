#![forbid(unsafe_code)]
use chrono::{TimeZone, Utc};
use color_eyre::eyre::{Context, ContextCompat, Result};
use exterminator::Exterminator;
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
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{info, info_span, warn};

use crate::{
    settings::Settings,
    state::{LOCKS, TIMERS, TimerData},
};

pub struct PodWatcher {
    client: Client,
    settings: Arc<Settings>,
    exterminator: Arc<Exterminator>,
}

impl PodWatcher {
    pub fn new(client: Client, settings: Arc<Settings>, exterminator: Arc<Exterminator>) -> Self {
        Self {
            client,
            settings,
            exterminator,
        }
    }

    pub async fn watch_pods(&self) -> Result<()> {
        let pods: Api<Pod> = Api::default_namespaced(self.client.clone());

        let selector = format!("{}/enable=true", self.settings.extermination.labels_prefix);
        let params = ListParams::default().labels(&selector);

        info!("Listing managed pods that were created before startup");

        let managed_pods = pods.list(&params).await?;
        for pod in managed_pods {
            self.handle_pod_safe(pod).await;
        }

        info!("Starting watcher");

        let watcher = runtime::watcher(pods, Config::default().labels(&selector));
        let mut watcher = Box::pin(watcher);

        while let Some(event) = watcher.try_next().await? {
            if let Event::Apply(pod) = event {
                self.handle_pod_safe(pod).await;
            }
        }

        Ok(())
    }

    /// This function won't return an error
    async fn handle_pod_safe(&self, pod: Pod) {
        let name = pod.metadata.name.as_deref().unwrap_or_default();
        let span = info_span!("pod", name);
        let _enter = span.enter();

        let handle_result = self.handle_pod(pod).await;
        match handle_result {
            Ok(_) => {
                info!("Pod handled successfully");
            }
            Err(err) => {
                warn!("Pod generated error: {}", err);
            }
        }
    }

    async fn handle_pod(&self, pod: Pod) -> Result<()> {
        let pod_name = pod.metadata.name.wrap_err("Missing pod name")?;

        info!("Handling pod");

        let expiry_label = format!("{}/expires-at", self.settings.extermination.labels_prefix);
        let expires_at = pod
            .metadata
            .labels
            .and_then(|labels| labels.get(&expiry_label).cloned())
            .wrap_err("No expiry time set")?;

        let expires_at = expires_at
            .parse::<u64>()
            .wrap_err("Invalid expires-at value")?;

        let already_handled = TIMERS.get(&pod_name);

        if let Some(handled) = already_handled {
            if handled.value().raw_value != expires_at {
                // Expiry time changed, we need to reset timer
                info!("Resetting timer");
                handled.value().handle.abort();
            } else {
                info!("Timer remains the same, skipping");
                return Ok(());
            }
        }

        let wait_time = duration_until(expires_at);

        let name = pod_name.clone();
        let exterminator = self.exterminator.clone();
        let timer = tokio::spawn(async move {
            let span = info_span!("pod_killer", name);
            let _enter = span.enter();

            if let Some(duration) = wait_time {
                info!("waiting for {duration:?}");
                sleep(duration).await;
            }

            if !LOCKS.insert(name.clone()) {
                // Already being handled
                info!("Pod is already being handled, skipping");
                return;
            }

            info!("Deleting pod");
            exterminator
                .exterminate(name.clone())
                .await
                .map_err(|e| {
                    warn!("Failed to exterminate pod: {}", e);
                })
                .ok();

            LOCKS.remove(&name);
        });

        let data = TimerData {
            handle: timer,
            raw_value: expires_at,
        };
        TIMERS.insert(pod_name.clone(), data);

        Ok(())
    }
}

fn duration_until(expires_at: u64) -> Option<Duration> {
    let expires_at_dt = Utc.timestamp_opt(expires_at as i64, 0).single()?;

    let diff = expires_at_dt.signed_duration_since(Utc::now());

    if diff.num_milliseconds() > 0 {
        Some(Duration::from_millis(diff.num_milliseconds() as u64))
    } else {
        None // already expired
    }
}
