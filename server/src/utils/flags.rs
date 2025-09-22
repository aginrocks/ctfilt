use manifests::ChallengeFlag;
use mongodb::bson::oid::ObjectId;
use sha2::{Digest, Sha256};

pub struct FlagGenerator {
    pub secret: String,
    pub flag_length: usize,
}

pub struct GeneratedFlag {
    pub meta: ChallengeFlag,
    pub value: String,
}

impl FlagGenerator {
    pub fn new(secret: String, flag_length: usize) -> Self {
        Self {
            secret,
            flag_length,
        }
    }

    pub fn generate(&self, user_id: ObjectId, challenge_id: ObjectId, flag_slug: &str) -> String {
        let seed = format!("{user_id}:{challenge_id}:{}:{flag_slug}", self.secret);

        let flag = format!("{:x}", Sha256::digest(seed));
        let flag = &flag[..self.flag_length];

        format!("Linus{{{flag}}}")
    }

    pub fn generate_all(
        &self,
        user_id: ObjectId,
        challenge_id: ObjectId,
        flags: Vec<ChallengeFlag>,
    ) -> Vec<GeneratedFlag> {
        flags
            .into_iter()
            .map(|flag_meta| match flag_meta.spec {
                manifests::ChallengeFlagMeta::Static { ref flag } => GeneratedFlag {
                    meta: flag_meta.clone(),
                    value: flag.clone(),
                },
                manifests::ChallengeFlagMeta::DynamicMount { .. } => {
                    let value = self.generate(user_id, challenge_id, &flag_meta.slug);
                    GeneratedFlag {
                        meta: flag_meta,
                        value,
                    }
                }
            })
            .collect()
    }
}
