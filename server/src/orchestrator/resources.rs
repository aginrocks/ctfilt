use std::sync::Arc;

use chrono::{Duration, Utc};
use color_eyre::eyre::{Context, ContextCompat, Result};
use headscale::{
    apis::{
        configuration::Configuration,
        headscale_service_api::{
            headscale_service_create_pre_auth_key, headscale_service_list_users,
        },
    },
    models::V1CreatePreAuthKeyRequest,
};
use kube::Client;

use crate::database::ChallengeMetadata;

pub struct ResourceProvisisoner {
    pub kube: Arc<Client>,
    pub headscale_config: Arc<Configuration>,
    pub metadata: ChallengeMetadata,
    pub subject: String,
}

impl ResourceProvisisoner {
    pub fn new(
        kube: Arc<Client>,
        headscale_config: Arc<Configuration>,
        metadata: ChallengeMetadata,
        subject: String,
    ) -> Self {
        Self {
            kube,
            headscale_config,
            metadata,
            subject,
        }
    }

    pub async fn generate_preauth_key(&self) -> Result<String> {
        // TODO: Cache user mappings in Redis
        let users = headscale_service_list_users(&self.headscale_config, None, None, None)
            .await
            .wrap_err("Failed to fetch VPN users")?;

        let user_id = users
            .users
            .wrap_err("No users found in VPN")?
            .into_iter()
            .find_map(|user| {
                user.provider_id
                    .map(|id_url| id_url.split('/').next_back().unwrap_or("").to_string())
                    .and_then(|id| if id == self.subject { user.id } else { None })
            })
            .wrap_err("User does not exist in VPN")?;

        let exp = Utc::now() + Duration::hours(1);

        let options = V1CreatePreAuthKeyRequest {
            user: Some(user_id),
            ephemeral: Some(true),
            reusable: Some(false),
            expiration: Some(exp.to_rfc3339()),
            ..Default::default()
        };
        let response = headscale_service_create_pre_auth_key(&self.headscale_config, options)
            .await
            .wrap_err("Failed to create preauth key")?;

        let key = response
            .pre_auth_key
            .wrap_err("Missing preauth key")?
            .key
            .wrap_err("Missing preauth key")?;

        Ok(key)
    }
}
