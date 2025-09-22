use mongodb::bson::oid::ObjectId;
use sha2::{Digest, Sha256};

pub struct FlagGenerator {
    pub secret: String,
    pub flag_length: usize,
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
}
