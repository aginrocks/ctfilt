use color_eyre::eyre::Result;

use crate::database::ChallengeMetadata;

use super::ChallengeOrchestrator;

impl ChallengeOrchestrator {
    pub async fn start_challenge(&self, metadata: &ChallengeMetadata, subject: &str) -> Result<()> {
        Ok(())
    }
}
