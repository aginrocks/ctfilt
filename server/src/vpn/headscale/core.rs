use async_trait::async_trait;
use chrono::{Duration, Utc};
use color_eyre::{
    Result,
    eyre::{Context, ContextCompat},
};
use headscale::{
    apis::headscale_service_api::{
        headscale_service_create_pre_auth_key, headscale_service_list_users,
    },
    models::V1CreatePreAuthKeyRequest,
};
use k8s_openapi::api::core::v1::{
    Capabilities, Container, EnvVar, EnvVarSource, ObjectFieldSelector, SecretKeySelector,
    SecurityContext,
};

use crate::vpn::{VpnCore, headscale::HeadscaleClient};

#[async_trait]
impl VpnCore for HeadscaleClient {
    fn get_sidecar(&self, secret_name: &str) -> Container {
        // TODO: Migrate to configmaps
        let env = vec![
            EnvVar {
                name: "TS_KUBE_SECRET".to_string(),
                value: Some("".to_string()),
                ..Default::default()
            },
            EnvVar {
                name: "TS_STATE_DIR".to_string(),
                value: Some("/tmp".to_string()),
                ..Default::default()
            },
            EnvVar {
                name: "TS_EPHEMERAL".to_string(),
                value: Some("true".to_string()),
                ..Default::default()
            },
            EnvVar {
                name: "TS_USERSPACE".to_string(),
                value: Some("false".to_string()),
                ..Default::default()
            },
            EnvVar {
                name: "TS_DEBUG_FIREWALL_MODE".to_string(),
                value: Some("auto".to_string()),
                ..Default::default()
            },
            EnvVar {
                name: "TS_AUTHKEY".to_string(),
                value_from: Some(EnvVarSource {
                    secret_key_ref: Some(SecretKeySelector {
                        name: secret_name.to_string(),
                        key: "TS_AUTHKEY".to_string(),
                        optional: Some(true),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            EnvVar {
                name: "TS_EXTRA_ARGS".to_string(),
                value: Some(format!("--login-server={}", self.public_url)),
                ..Default::default()
            },
            EnvVar {
                name: "POD_NAME".to_string(),
                value_from: Some(EnvVarSource {
                    field_ref: Some(ObjectFieldSelector {
                        field_path: "metadata.name".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            EnvVar {
                name: "POD_UID".to_string(),
                value_from: Some(EnvVarSource {
                    field_ref: Some(ObjectFieldSelector {
                        field_path: "metadata.uid".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ];

        Container {
            name: "ts-sidecar".to_string(),
            image: Some("ghcr.io/tailscale/tailscale:latest".to_string()),
            env: Some(env),
            security_context: Some(SecurityContext {
                capabilities: Some(Capabilities {
                    add: Some(vec!["NET_ADMIN".to_string()]),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    async fn generate_key(&self, subject: &str) -> Result<String> {
        // TODO: Cache user mappings in Redis
        let users = headscale_service_list_users(&self.config, None, None, None)
            .await
            .wrap_err("Failed to fetch VPN users")?;

        let user_id = users
            .users
            .wrap_err("No users found in VPN")?
            .into_iter()
            .find_map(|user| {
                user.provider_id
                    .map(|id_url| id_url.split('/').next_back().unwrap_or("").to_string())
                    .and_then(|id| if id == subject { user.id } else { None })
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
        let response = headscale_service_create_pre_auth_key(&self.config, options)
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
