use chrono::Utc;
use color_eyre::eyre::Result;
use mongodb::bson::oid::ObjectId;

use crate::{
    database::{ChallengeFlagMeta, ChallengeMetadata},
    orchestrator::{
        ChallengeStatus, RunningChallenge, RunningChallengeBuilder,
        resources::{DynamicFlag, ResourceProvisisonerBuilder},
    },
    utils::generate_hostname,
};

use super::ChallengeOrchestrator;

impl ChallengeOrchestrator {
    pub async fn start_challenge(
        &self,
        id: ObjectId,
        metadata: &ChallengeMetadata,
        user_id: ObjectId,
        subject: &str,
    ) -> Result<RunningChallenge> {
        let hostname = generate_hostname()?;

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

        dbg!(key);

        // Generating flags
        let flags = self.generate_flags(id, metadata, user_id);
        let secret_name = provisioner.provision_flags_secret(flags.clone()).await?;

        // TODO: Add expiry

        // Creating a Deployment
        let hostname = provisioner
            .provision_challenge_pod(&ts_secret_name, &secret_name, flags)
            .await?;

        let response = RunningChallengeBuilder::default()
            .id(id)
            .ip(None)
            .expires_at(Utc::now())
            .hostname(Some(hostname))
            .status(ChallengeStatus::Starting)
            .build()?;
        Ok(response)
    }

    pub fn generate_flags(
        &self,
        challenge_id: ObjectId,
        metadata: &ChallengeMetadata,
        user_id: ObjectId,
    ) -> Vec<DynamicFlag> {
        metadata
            .flags
            .iter()
            .enumerate()
            .filter_map(|(index, flag)| match flag.meta {
                ChallengeFlagMeta::DynamicMount { ref mount_path } => Some(DynamicFlag {
                    flag: self.flags.generate(user_id, challenge_id, index),
                    mount_path: mount_path.clone(),
                }),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}
