use color_eyre::eyre::Result;

use crate::{
    database::ChallengeMetadata,
    orchestrator::{RunningChallenge, RunningChallengeBuilder, resources::ResourceProvisisoner},
};

use super::ChallengeOrchestrator;

impl ChallengeOrchestrator {
    pub async fn start_challenge(
        &self,
        metadata: &ChallengeMetadata,
        subject: &str,
    ) -> Result<RunningChallenge> {
        let provisioner =
            ResourceProvisisoner::new(self.kube.clone(), metadata.clone(), subject.to_string());

        let response = RunningChallengeBuilder::default().build()?;
        Ok(response)
    }
}
