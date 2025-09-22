use std::collections::HashMap;

use chrono::DateTime;
use color_eyre::eyre::{Context, ContextCompat, Result, eyre};
use dashmap::DashMap;
use exterminator::Extermination;
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
use mongodb::bson::oid::ObjectId;
use serde::Serialize;
use tokio::sync::broadcast::{self, Sender};
use tracing::{info, info_span, warn};

use crate::orchestrator::{ChallengeStatus, RunningChallenge};

#[derive(Clone)]
pub struct UserState {
    pub challenges: HashMap<ObjectId, RunningChallenge>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PodEvent {
    pub user: ObjectId,
    pub challenges: Vec<RunningChallenge>,
}

pub struct PodWatcher {
    pub client: Client,
    pub users_state: DashMap<ObjectId, UserState>,
    pub extermination: Extermination,
    pub sender: Sender<PodEvent>,
}

impl PodWatcher {
    pub fn new(client: Client, extermination: Extermination) -> Self {
        let (sender, _) = broadcast::channel(64);
        Self {
            client,
            users_state: DashMap::new(),
            extermination,
            sender,
        }
    }

    pub async fn watch_pods(&self) -> Result<()> {
        let pods: Api<Pod> = Api::default_namespaced(self.client.clone());

        let selector = "app=ctfilt-challenge";
        let params = ListParams::default().labels(selector);

        info!("Reading cluster state");

        let challenge_pods = pods.list(&params).await?;
        for pod in challenge_pods {
            self.handle_pod_safe(pod).await;
        }

        info!("Starting watcher");

        let watcher = runtime::watcher(pods, Config::default().labels(selector));
        let mut watcher = Box::pin(watcher);

        while let Some(event) = watcher.try_next().await? {
            match event {
                Event::Apply(pod) => {
                    self.handle_pod_safe(pod).await;
                }
                Event::Delete(pod) => {
                    self.handle_pod_deletion_safe(pod).await;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// This function won't return an error
    async fn handle_pod_safe(&self, pod: Pod) {
        // Handle the pod safely, logging any errors internally
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

    /// This function won't return an error
    async fn handle_pod_deletion_safe(&self, pod: Pod) {
        // Handle the deletion safely, logging any errors internally
        let name = pod.metadata.name.as_deref().unwrap_or_default();
        let span = info_span!("pod_del", name);
        let _enter = span.enter();

        let handle_result = self.handle_pod_deletion(pod).await;
        match handle_result {
            Ok(_) => {
                info!("Pod handled successfully");
            }
            Err(err) => {
                warn!("Pod generated error: {}", err);
            }
        }
    }

    fn summarize_pod(&self, pod: Pod) -> Result<(ObjectId, RunningChallenge)> {
        let pod_name = pod.metadata.name.wrap_err("Missing pod name")?;

        let labels = pod.metadata.labels.wrap_err("Missing pod labels")?;
        let expiry_label = format!("{}/expires-at", self.extermination.labels_prefix);

        let challenge = labels
            .get("challenge")
            .wrap_err("Missing challenge label")?;
        let challenge =
            ObjectId::parse_str(challenge).wrap_err("Invalid challenge label, not an ObjectId")?;

        let user = labels.get("user").wrap_err("Missing user label")?;
        let user = ObjectId::parse_str(user).wrap_err("Invalid user label, not an ObjectId")?;

        let expires_at = labels.get(&expiry_label).wrap_err("Missing expiry label")?;
        let expires_at = expires_at
            .parse::<i64>()
            .wrap_err("Invalid expires-at value")?;
        let expires_at =
            DateTime::from_timestamp(expires_at, 0).wrap_err("Invalid expires-at value")?;

        let status = pod.status.wrap_err("Missing pod status")?;
        dbg!(&status.phase);
        let status = match status.phase.wrap_err("Missing pod phase")?.as_str() {
            "Pending" => ChallengeStatus::Starting,
            "Running" => ChallengeStatus::Running,
            "Succeeded" | "Failed" => ChallengeStatus::Stopping,
            _ => ChallengeStatus::Unknown,
        };

        Ok((
            user,
            RunningChallenge {
                id: challenge,
                status,
                hostname: Some(pod_name),
                ip: Some("".to_string()),
                expires_at,
            },
        ))
    }

    async fn handle_pod(&self, pod: Pod) -> Result<()> {
        let (user, summary) = self.summarize_pod(pod)?;

        let mut user_state = self.users_state.entry(user).or_insert_with(|| UserState {
            challenges: HashMap::new(),
        });

        let old_state = user_state.challenges.insert(summary.id, summary.clone());
        if let Some(old_state) = old_state
            && old_state == summary
        {
            return Ok(());
        }

        drop(user_state);
        self.send_event(user)?;

        Ok(())
    }

    async fn handle_pod_deletion(&self, pod: Pod) -> Result<()> {
        let (user, summary) = self.summarize_pod(pod)?;

        let mut user_state = self.users_state.entry(user).or_insert_with(|| UserState {
            challenges: HashMap::new(),
        });

        user_state.challenges.remove(&summary.id);

        drop(user_state);
        self.send_event(user)?;

        Ok(())
    }

    pub fn get_latest_event(&self, user: ObjectId) -> Result<PodEvent> {
        let user_state = self.users_state.get(&user).wrap_err("User not found")?;

        let challenges = user_state.challenges.values().cloned().collect();

        Ok(PodEvent { user, challenges })
    }

    fn send_event(&self, user: ObjectId) -> Result<()> {
        let event = self.get_latest_event(user)?;

        self.sender
            .send(event)
            .map_err(|_| eyre!("Failed to send"))?;

        Ok(())
    }
}
