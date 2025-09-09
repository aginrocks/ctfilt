use color_eyre::eyre::Result;
use mongodb::bson::oid::ObjectId;

use crate::{
    database::ChallengeMetadata,
    orchestrator::{
        RunningChallenge, RunningChallengeBuilder,
        resources::{ResourceProvisisoner, ResourceProvisisonerBuilder},
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
            .build()?;

        // Setting up Tailscale access
        let key = provisioner.generate_preauth_key().await?;
        let ts_secret_name = provisioner
            .provision_tailscale_secret(&hostname, &key)
            .await?;

        let sa_name = provisioner
            .provision_tailscale_sa(&hostname, &ts_secret_name)
            .await?;

        dbg!(key);

        let response = RunningChallengeBuilder::default().id(id).build()?;
        Ok(response)
    }
}
