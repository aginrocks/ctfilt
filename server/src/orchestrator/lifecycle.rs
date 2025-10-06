use chrono::{Duration, Utc};
use color_eyre::eyre::{Context, ContextCompat, Result, bail, eyre};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    Api,
    api::{Patch, PatchParams},
};
use manifests::{ChallengeFlagMeta, ChallengeMetadata, ChallengeSpec};
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use tracing::{info, instrument};

use crate::{
    axum_error::{AxumError, AxumResult},
    orchestrator::{
        ChallengeStatus, RunningChallenge, RunningChallengeBuilder,
        resources::{DynamicFlag, ResourceProvisisonerBuilder},
    },
    utils::generate_hostname,
};

use super::ChallengeOrchestrator;

impl ChallengeOrchestrator {
    #[instrument(skip(self, metadata, subject, user_id))]
    pub async fn start_challenge(
        &self,
        id: ObjectId,
        metadata: &ChallengeMetadata,
        user_id: ObjectId,
        subject: &str,
    ) -> Result<RunningChallenge> {
        let containers = match metadata.spec {
            ChallengeSpec::Container { ref containers } => containers.clone(),
            _ => bail!("This challenge cannot be started"),
        };

        let hostname = generate_hostname()?;
        info!("Generated hostname: {hostname}");

        let provisioner = ResourceProvisisonerBuilder::default()
            .kube(self.kube.clone())
            .headscale_config(self.headscale_config.clone())
            .subject(subject.to_string())
            .challenge_id(id)
            .user_id(user_id)
            .hostname(hostname)
            .headscale_public_url(self.headscale_public_url.clone())
            .build()?;

        // Setting up Tailscale access
        let key = provisioner.generate_preauth_key().await?;
        let ts_secret_name = provisioner.provision_tailscale_secret(&key).await?;

        info!("Tailscale set up");

        // Generating flags
        let flags = self.generate_flags(id, metadata, user_id);
        let secret_name = provisioner.provision_flags_secret(flags.clone()).await?;

        info!("Flags generated");

        // TODO: Add expiry

        // Creating a Deployment
        let hostname = provisioner
            .provision_challenge_pod(&ts_secret_name, &secret_name, flags, containers)
            .await?;

        info!("Kubernetes resources provisioned");

        let response = RunningChallengeBuilder::default()
            .id(id)
            .ip(None)
            .expires_at(Utc::now())
            .hostname(Some(hostname))
            .status(ChallengeStatus::Starting)
            .build()?;
        Ok(response)
    }

    pub async fn add_time(
        &self,
        id: ObjectId,
        user_id: ObjectId,
        time: Duration,
    ) -> AxumResult<()> {
        let user_challenges = self
            .watcher
            .users_state
            .get(&user_id)
            .ok_or_else(|| AxumError::not_found(eyre!("Challenge is not running")))?;

        let challenge_data = user_challenges
            .value()
            .challenges
            .get(&id)
            .ok_or_else(|| AxumError::not_found(eyre!("Challenge is not running")))?;

        let exp = challenge_data.expires_at + time;

        let pods: Api<Pod> = Api::default_namespaced(self.kube.clone());

        let hostname = challenge_data
            .hostname
            .clone()
            .wrap_err("Missing hostname")?;

        let assassin_label = format!("{}/expires-at", self.exterminator.settings.labels_prefix);

        let patch = json!({
            "metadata": {
                "labels": {
                    assassin_label: exp.timestamp().to_string()
                },
            },
        });

        pods.patch_metadata(&hostname, &PatchParams::default(), &Patch::Merge(patch))
            .await
            .wrap_err("Failed to modify expiry date")?;

        Ok(())
    }

    pub fn generate_flags(
        &self,
        challenge_id: ObjectId,
        metadata: &ChallengeMetadata,
        user_id: ObjectId,
    ) -> Vec<DynamicFlag> {
        let generated = self
            .flags
            .generate_all(user_id, challenge_id, metadata.flags.clone());

        generated
            .iter()
            .filter_map(|flag| match flag.meta.spec {
                ChallengeFlagMeta::DynamicMount {
                    ref mount_path,
                    permissions,
                } => Some(DynamicFlag {
                    flag: flag.value.clone(),
                    mount_path: mount_path.clone(),
                    slug: flag.meta.slug.clone(),
                    permissions,
                }),
                _ => None,
            })
            .collect()
    }
}
