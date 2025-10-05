use rand::Rng;
use rand::distr::Alphanumeric;
use rand::rngs::ThreadRng;

pub fn generate_webhook_secret() -> String {
    ThreadRng::default()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect()
}
