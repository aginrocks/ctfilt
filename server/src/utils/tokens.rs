use color_eyre::eyre::Result;
use mongodb::Database;
use rand::{Rng, distr::Alphanumeric, rngs::ThreadRng};
use sha2::{Digest, Sha256};

use crate::database::PartialAccessToken;

pub fn generate_token() -> String {
    let rng = ThreadRng::default();

    let token: String = rng
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();

    format!("ctfilt_{token}")
}

pub fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token))
}

pub async fn create_token(database: &Database) -> Result<String> {
    let token = generate_token();
    let hashed_token = hash_token(&token);

    database
        .collection::<PartialAccessToken>("tokens")
        .insert_one(PartialAccessToken { hashed_token })
        .await?;

    println!("{token}");

    Ok(token)
}
