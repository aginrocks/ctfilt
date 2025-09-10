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
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Capabilities, Container, EnvVar, EnvVarSource, ObjectFieldSelector, PodSpec,
            PodTemplateSpec, ResourceFieldSelector, Secret, SecretKeySelector, SecurityContext,
            ServiceAccount,
        },
        rbac::v1::{PolicyRule, Role, RoleBinding, RoleRef, Subject},
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::{Api, Client, api::ObjectMeta};
use mongodb::bson::oid::ObjectId;

use crate::database::ChallengeMetadata;

#[derive(Builder, Clone)]
pub struct ResourceProvisisoner {
    pub kube: Client,
    pub headscale_config: Arc<Configuration>,
    pub headscale_public_url: String,
    pub metadata: ChallengeMetadata,
    pub subject: String,
    pub challenge_id: ObjectId,
    pub user_id: ObjectId,
    pub hostname: String,
}

pub struct DynamicFlag {
    pub flag: String,
    pub mount_path: String,
}

impl ResourceProvisisoner {
    pub fn get_labels(&self) -> BTreeMap<String, String> {
        BTreeMap::from([
            ("app".to_string(), "ctfilt-challenge".to_string()),
            ("user".to_string(), self.user_id.to_string()),
            ("user-sub".to_string(), self.subject.clone()),
            ("challenge".to_string(), self.challenge_id.to_string()),
        ])
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
                labels: Some(self.get_labels()),
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

    /// Provisions a ServiceAccount for Tailscale access
    pub async fn provision_tailscale_sa(&self, secret_name: &str) -> Result<String> {
        let sa_name = format!("ts-sa-{}", self.hostname);
        let role_name = format!("ts-role-{}", self.hostname);
        let binding_name = format!("ts-binding-{}", self.hostname);

        // Create a ServiceAccount
        let service_accounts: Api<ServiceAccount> = Api::default_namespaced(self.kube.clone());
        let sa = ServiceAccount {
            metadata: ObjectMeta {
                name: Some(sa_name.clone()),
                labels: Some(self.get_labels()),
                ..Default::default()
            },
            ..Default::default()
        };

        service_accounts
            .create(&Default::default(), &sa)
            .await
            .wrap_err("Failed to create Tailscale sa")?;

        // Create a role
        let roles: Api<Role> = Api::default_namespaced(self.kube.clone());
        let role = Role {
            metadata: ObjectMeta {
                name: Some(role_name.clone()),
                labels: Some(self.get_labels()),
                ..Default::default()
            },
            rules: Some(vec![PolicyRule {
                api_groups: Some(vec!["".to_string()]),
                resources: Some(vec!["secrets".to_string()]),
                resource_names: Some(vec![secret_name.to_string()]),
                verbs: vec!["get".to_string(), "patch".to_string(), "update".to_string()],
                ..Default::default()
            }]),
        };

        roles
            .create(&Default::default(), &role)
            .await
            .wrap_err("Failed to create Tailscale role")?;

        // Create a RoleBinding
        let bindings: Api<RoleBinding> = Api::default_namespaced(self.kube.clone());
        let binding = RoleBinding {
            metadata: ObjectMeta {
                name: Some(binding_name.clone()),
                labels: Some(self.get_labels()),
                ..Default::default()
            },
            subjects: Some(vec![Subject {
                kind: "ServiceAccount".to_string(),
                name: sa_name.clone(),
                ..Default::default()
            }]),
            role_ref: RoleRef {
                kind: "Role".to_string(),
                name: role_name,
                api_group: "rbac.authorization.k8s.io".to_string(),
            },
        };

        bindings
            .create(&Default::default(), &binding)
            .await
            .wrap_err("Failed to create Tailscale role binding")?;

        Ok(sa_name)
    }

    /// Provisions a Secret with flags for the challenge
    pub async fn provision_flags_secret(&self, flags: &Vec<DynamicFlag>) -> Result<String> {
        let secret_name = format!("flags-{}", self.hostname);
        let secrets: Api<Secret> = Api::default_namespaced(self.kube.clone());

        let flags_data = flags
            .iter()
            .enumerate()
            .map(|(index, flag)| (index.to_string(), flag.flag.clone()))
            .collect::<BTreeMap<_, _>>();

        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(secret_name.clone()),
                labels: Some(self.get_labels()),
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
                value: Some(ts_secret_name.to_string()),
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
    pub async fn provision_challenge_deployment(
        &self,
        ts_secret_name: &str,
        flags_secret_name: &str,
        sa_name: &str,
    ) -> Result<String> {
        let deployments: Api<Deployment> = Api::default_namespaced(self.kube.clone());

        let deployment = Deployment {
            metadata: ObjectMeta {
                name: Some(self.hostname.clone()),
                labels: Some(self.get_labels()),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                selector: LabelSelector {
                    match_labels: Some(self.get_labels()),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some(self.get_labels()),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        service_account_name: Some(sa_name.to_string()),
                        containers: vec![self.get_tailscale_sidecar(ts_secret_name)],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        deployments
            .create(&Default::default(), &deployment)
            .await
            .wrap_err("Failed to create challenge deployment")?;

        Ok(self.hostname.clone())
    }
}
