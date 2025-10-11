use std::collections::HashMap;

use headscale::{
    apis::{configuration::Configuration, headscale_service_api::headscale_service_list_nodes},
    models::V1Node,
};
use tokio::time::{Duration, interval};

use color_eyre::Result;
use fred::{
    prelude::{KeysInterface, Pool},
    types::Expiration,
};
use tracing::{debug, error, info, instrument, warn};

use crate::settings::Settings;

pub async fn poll(settings: &Settings, redis: Pool, config: Configuration) {
    let mut interval = interval(Duration::from_millis(settings.polling.interval_millis));

    loop {
        interval.tick().await;
        if let Err(e) = sync_data(redis.clone(), &config).await {
            error!("Failed to poll data: {e:?}");
        }
    }
}

#[instrument(skip(redis, config))]
pub async fn sync_data(redis: Pool, config: &Configuration) -> Result<()> {
    info!("Attempting sync");

    // Fetching nodes from Headscale
    let nodes = headscale_service_list_nodes(config, None).await?;
    let nodes_list = nodes.nodes.unwrap_or_default();

    let nodes_count = nodes_list.len();

    debug!("Fetched {nodes_count} nodes from Headscale");

    let mut nodes_map: HashMap<String, Vec<V1Node>> = HashMap::new();

    // Grouping nodes by user
    for node in nodes_list {
        let Some(user) = node.user.clone() else {
            warn!("Skipping a node with no user");
            continue;
        };
        let Some(user_id) = user.provider_id.clone() else {
            warn!("Skipping a node with no external user ID");
            continue;
        };
        let user_id = user_id.split('/').next_back().unwrap_or(&user_id);

        let key = format!("hs:user:{user_id}:nodes");

        nodes_map.entry(key).or_default().push(node);
    }

    let users_count = nodes_map.len();

    let pipeline = redis.next_connected().pipeline();
    for (user_id, nodes) in nodes_map {
        let serialized = serde_json::to_string(&nodes)?;

        let _: () = pipeline
            .set(&user_id, serialized, Some(Expiration::EX(30)), None, false)
            .await?;
    }
    let _: () = pipeline.all().await?;

    info!("Synced {nodes_count} nodes across {users_count} users");

    Ok(())
}
