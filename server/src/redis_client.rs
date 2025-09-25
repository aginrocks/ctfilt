use color_eyre::eyre::Result;
use fred::prelude::{ClientLike, Config, Pool};

use crate::settings::Settings;

pub async fn init_redis(settings: &Settings) -> Result<Pool> {
    let config = Config::from_url(&settings.redis.connection_string)?;
    let pool = Pool::new(config, None, None, None, 6)?;

    let _redis_conn = pool.connect();
    pool.wait_for_connect().await?;

    Ok(pool)
}
