use headscale::apis::configuration::Configuration;
use tokio::time::{Duration, interval};

use color_eyre::Result;
use fred::prelude::Pool;
use tracing::error;

use crate::settings::Settings;

pub async fn poll(settings: &Settings, redis: Pool, headscale: Configuration) {
    let mut interval = interval(Duration::from_millis(settings.polling.interval_millis));

    loop {
        interval.tick().await;
        if let Err(e) = sync_data(redis.clone(), &headscale).await {
            error!("Failed to poll data: {e:?}");
        }
    }
}

pub async fn sync_data(redis: Pool, headscale: &Configuration) -> Result<()> {
    Ok(())
}
