use std::sync::Arc;

use kube::Client;

use crate::database::ChallengeMetadata;

pub struct ResourceProvisisoner {
    pub kube: Arc<Client>,
    pub metadata: ChallengeMetadata,
    pub subject: String,
}

impl ResourceProvisisoner {
    pub fn new(kube: Arc<Client>, metadata: ChallengeMetadata, subject: String) -> Self {
        Self {
            kube,
            metadata,
            subject,
        }
    }
}
