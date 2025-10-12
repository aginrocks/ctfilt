use std::collections::HashMap;

use headscale::{
    apis::{configuration::Configuration, headscale_service_api::headscale_service_list_nodes},
    models::V1Node,
};
use headscale_common::NodesChangedMessage;
use tokio::time::{Duration, interval};

use color_eyre::Result;
use fred::prelude::{KeysInterface, Pool, PubsubInterface};
use tracing::{debug, error, info, instrument, warn};

use crate::settings::Settings;

pub async fn poll(settings: &Settings, redis: Pool, config: Configuration) {
    let mut interval = interval(Duration::from_millis(settings.polling.interval_millis));

    loop {
        interval.tick().await;
        if let Err(e) = sync_data(redis.clone(), &config).await {
            error!(error = ?e, "Failed to poll data");
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

        nodes_map.entry(user_id.into()).or_default().push(node);
    }

    let users_count = nodes_map.len();

    update_cache_and_publish(&redis, nodes_map).await?;

    info!("Synced {nodes_count} nodes across {users_count} users");

    Ok(())
}

async fn update_cache_and_publish(
    redis: &Pool,
    nodes_map: HashMap<String, Vec<V1Node>>,
) -> Result<()> {
    for (user_id, nodes) in nodes_map {
        let serialized = serde_json::to_string(&nodes)?;

        let topic = format!("vpn:user:{}:nodes", user_id);
        let old_value: Option<String> = redis.getset(&topic, &serialized).await?;

        let _: () = redis.expire(&topic, 30, None).await?;

        // Publish only if the value actually changed
        if old_value.as_deref() != Some(&serialized) {
            debug!(user_id = %user_id, "Nodes changed");
            let message = NodesChangedMessage {
                user: user_id.clone(),
                nodes: nodes.clone(),
            };
            let serialized = serde_json::to_string(&message)?;
            let _: i64 = redis.next().publish("vpn:nodes", serialized).await?;
        }
    }

    Ok(())
}
