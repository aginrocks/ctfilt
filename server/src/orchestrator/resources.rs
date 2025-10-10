use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use chrono::{Duration, Utc};
use color_eyre::eyre::{Context, Result};
use derive_builder::Builder;
use k8s_openapi::api::core::v1::{
    Container, KeyToPath, LocalObjectReference, Pod, PodSpec, Secret, SecretVolumeSource, Volume,
    VolumeMount,
};
use kube::{Api, Client, api::ObjectMeta};
use manifests::ChallengeContainer;
use mongodb::bson::oid::ObjectId;

use crate::vpn::Vpn;

#[derive(Builder, Clone)]
pub struct ResourceProvisisoner {
    pub kube: Client,
    pub subject: String,
    pub challenge_id: ObjectId,
    pub user_id: ObjectId,
    pub hostname: String,
    pub vpn: Arc<dyn Vpn>,
}

#[derive(Clone)]
pub struct DynamicFlag {
    pub slug: String,
    pub flag: String,
    pub mount_path: String,
    pub container: Option<String>,
    pub permissions: Option<i32>,
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
        self.vpn.generate_key(&self.subject).await
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

    /// Provisisons a Deployment for the challenge
    pub async fn provision_challenge_pod(
        &self,
        vpn_secret_name: &str,
        flags_secret_name: &str,
        flags: Vec<DynamicFlag>,
        spec_containers: Vec<ChallengeContainer>,
    ) -> Result<String> {
        let pods: Api<Pod> = Api::default_namespaced(self.kube.clone());

        // TODO: Add support for mounting flags to specific containers (for now they will be mounted to all containers)
        // TODO: Add a challenge container and mount flags

        let mut flag_mounts: HashMap<String, Vec<VolumeMount>> = HashMap::new();

        for flag in &flags {
            flag_mounts
                .entry(flag.container.clone().unwrap_or("_default".to_string()))
                .or_default()
                .push(VolumeMount {
                    name: "flag".to_string(),
                    mount_path: flag.mount_path.clone(),
                    sub_path: Some(flag.slug.clone()),
                    read_only: Some(true),
                    ..Default::default()
                })
        }

        let containers = spec_containers
            .into_iter()
            .map(|c| {
                let mounts = [
                    flag_mounts.get(&c.name).cloned().unwrap_or_default(),
                    flag_mounts.get("_default").cloned().unwrap_or_default(),
                ]
                .concat();

                Container {
                    name: c.name,
                    image: Some(c.image),
                    args: c.args,
                    command: c.command,
                    volume_mounts: Some(mounts),
                    // TODO: Remove when proper versioning is in place
                    image_pull_policy: Some("Always".to_string()),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();

        let flag_sources = flags
            .iter()
            .map(|f| KeyToPath {
                key: f.slug.clone(),
                path: f.slug.clone(),
                mode: Some(f.permissions.unwrap_or(0o600)),
            })
            .collect();

        let pod = Pod {
            metadata: ObjectMeta {
                name: Some(self.hostname.clone()),
                labels: Some(self.get_labels(LabelResourceType::Pod)),
                ..Default::default()
            },
            spec: Some(PodSpec {
                containers: [
                    containers.as_slice(),
                    &[self.vpn.get_sidecar(vpn_secret_name)],
                ]
                .concat(),
                volumes: Some(vec![Volume {
                    name: "flag".to_string(),
                    secret: Some(SecretVolumeSource {
                        secret_name: Some(flags_secret_name.to_string()),
                        default_mode: Some(0o600),
                        items: Some(flag_sources),
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
