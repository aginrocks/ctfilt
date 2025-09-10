use color_eyre::eyre::Result;
use mongodb::bson::oid::ObjectId;

use crate::{
    database::{ChallengeFlagMeta, ChallengeMetadata},
    orchestrator::{
        RunningChallenge, RunningChallengeBuilder,
        resources::{DynamicFlag, ResourceProvisisoner, ResourceProvisisonerBuilder},
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
            .metadata(metadata.clone())
            .subject(subject.to_string())
            .challenge_id(id)
            .user_id(user_id)
            .hostname(hostname)
            .headscale_public_url(self.headscale_public_url.clone())
            .build()?;

        // Setting up Tailscale access
        let key = provisioner.generate_preauth_key().await?;
        let ts_secret_name = provisioner.provision_tailscale_secret(&key).await?;

        let sa_name = provisioner.provision_tailscale_sa(&ts_secret_name).await?;

        dbg!(key);

        // Generating flags
        let flags = self.generate_flags(id, metadata, user_id);
        let secret_name = provisioner.provision_flags_secret(&flags).await?;

        // Creating a Deployment

        let response = RunningChallengeBuilder::default().id(id).build()?;
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
                ChallengeFlagMeta::Dynamic { ref mount_path } => Some(DynamicFlag {
                    flag: self.flags.generate(user_id, challenge_id, index),
                    mount_path: mount_path.clone(),
                }),
                _ => None,
            })
            .collect::<Vec<_>>()
    }
}
