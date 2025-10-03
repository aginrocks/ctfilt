use std::{collections::BTreeMap, sync::Arc};

use chrono::{Duration, Utc};
use color_eyre::eyre::{Context, ContextCompat, Result};
use derive_builder::Builder;
use headscale::{
    apis::{
        configuration::Configuration,
        headscale_service_api::{
            headscale_service_create_pre_auth_key, headscale_service_list_users,
        },
    },
    models::V1CreatePreAuthKeyRequest,
};
use k8s_openapi::api::core::v1::{
    Capabilities, Container, EnvVar, EnvVarSource, LocalObjectReference, ObjectFieldSelector, Pod,
    PodSpec, Secret, SecretKeySelector, SecretVolumeSource, SecurityContext, Volume, VolumeMount,
};
use kube::{Api, Client, api::ObjectMeta};
use manifests::ChallengeContainer;
use mongodb::bson::oid::ObjectId;

#[derive(Builder, Clone)]
pub struct ResourceProvisisoner {
    pub kube: Client,
    pub headscale_config: Arc<Configuration>,
    pub headscale_public_url: String,
    pub subject: String,
    pub challenge_id: ObjectId,
    pub user_id: ObjectId,
    pub hostname: String,
}

#[derive(Clone)]
pub struct DynamicFlag {
    pub slug: String,
    pub flag: String,
    pub mount_path: String,
}

pub enum LabelResourceType {
    Pod,
    Other,
}

impl ResourceProvisisoner {
    pub fn get_labels(&self, r#type: LabelResourceType) -> BTreeMap<String, String> {
        let mut labels = BTreeMap::new();

        labels.insert("app".to_string(), "ctfilt-challenge".to_string());
        labels.insert("user".to_string(), self.user_id.to_string());
        labels.insert("user-sub".to_string(), self.subject.clone());
        labels.insert("challenge".to_string(), self.challenge_id.to_string());
        // TODO: Make labels dynamic
        labels.insert(
            "pod-assassin.agin.rocks/enable".to_string(),
            "true".to_string(),
        );

        match r#type {
            LabelResourceType::Pod => {
                labels.insert(
                    "pod-assassin.agin.rocks/expires-at".to_string(),
                    (Utc::now() + Duration::hours(1)).timestamp().to_string(),
                );
            }
            LabelResourceType::Other => {
                labels.insert(
                    "pod-assassin.agin.rocks/pod".to_string(),
                    self.hostname.clone(),
                );
            }
        }

        labels
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

    /// Creates a Tailscale secret in the cluster and returns its name
    pub async fn provision_tailscale_secret(&self, preauth_key: &str) -> Result<String> {
        let secret_name = format!("ts-secret-{}", self.hostname);

        let secrets: Api<Secret> = Api::default_namespaced(self.kube.clone());
        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(secret_name.clone()),
                labels: Some(self.get_labels(LabelResourceType::Other)),
                ..Default::default()
            },
            string_data: Some(BTreeMap::from([(
                "TS_AUTHKEY".to_string(),
                preauth_key.to_string(),
            )])),
            ..Default::default()
        };

        secrets
            .create(&Default::default(), &secret)
            .await
            .wrap_err("Failed to create Tailscale secret")?;

        Ok(secret_name)
    }

    /// Provisions a Secret with flags for the challenge
    pub async fn provision_flags_secret(&self, flags: Vec<DynamicFlag>) -> Result<String> {
        let secret_name = format!("flags-{}", self.hostname);
        let secrets: Api<Secret> = Api::default_namespaced(self.kube.clone());

        let flags_data = flags
            .iter()
            .map(|flag| (flag.slug.clone(), flag.flag.clone()))
            .collect::<BTreeMap<_, _>>();

        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(secret_name.clone()),
                labels: Some(self.get_labels(LabelResourceType::Other)),
                ..Default::default()
            },
            string_data: Some(flags_data),
            ..Default::default()
        };

        secrets
            .create(&Default::default(), &secret)
            .await
            .wrap_err("Failed to create flags secret")?;

        Ok(secret_name)
    }

    /// Crestes a Tailscale sidecar container spec
    pub fn get_tailscale_sidecar(&self, ts_secret_name: &str) -> Container {
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
                        name: ts_secret_name.to_string(),
                        key: "TS_AUTHKEY".to_string(),
                        optional: Some(true),
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            EnvVar {
                name: "TS_EXTRA_ARGS".to_string(),
                value: Some(format!("--login-server={}", self.headscale_public_url)),
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

    /// Provisisons a Deployment for the challenge
    pub async fn provision_challenge_pod(
        &self,
        ts_secret_name: &str,
        flags_secret_name: &str,
        flags: Vec<DynamicFlag>,
        spec_containers: Vec<ChallengeContainer>,
    ) -> Result<String> {
        let pods: Api<Pod> = Api::default_namespaced(self.kube.clone());

        // TODO: Add support for mounting flags to specific containers (for now they will be mounted to all containers)
        // TODO: Add a challenge container and mount flags

        let flag_mounts = flags
            .into_iter()
            .map(|flag| VolumeMount {
                name: "flag".to_string(),
                mount_path: flag.mount_path.clone(),
                sub_path: Some(flag.slug),
                read_only: Some(true),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let containers = spec_containers
            .into_iter()
            .map(|c| Container {
                name: c.name,
                image: Some(c.image),
                args: c.args,
                command: c.command,
                volume_mounts: Some(flag_mounts.clone()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        let pod = Pod {
            metadata: ObjectMeta {
                name: Some(self.hostname.clone()),
                labels: Some(self.get_labels(LabelResourceType::Pod)),
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: [
                    containers.as_slice(),
                    &[self.get_tailscale_sidecar(ts_secret_name)],
                ]
                .concat(),
                volumes: Some(vec![Volume {
                    name: "flag".to_string(),
                    secret: Some(SecretVolumeSource {
                        secret_name: Some(flags_secret_name.to_string()),
                        default_mode: Some(0o600),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
                // TODO: Replace with Harbor
                image_pull_secrets: Some(vec![LocalObjectReference {
                    name: "forgejo-secret".to_string(),
                }]),
                ..Default::default()
            }),

            ..Default::default()
        };

        pods.create(&Default::default(), &pod)
            .await
            .wrap_err("Failed to create challenge deployment")?;

        Ok(self.hostname.clone())
    }
}
