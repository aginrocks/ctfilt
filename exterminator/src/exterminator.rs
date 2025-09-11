use color_eyre::eyre::{ContextCompat, Result, bail};
use k8s_openapi::api::{
    core::v1::{ConfigMap, Pod, Secret, ServiceAccount},
    rbac::v1::{Role, RoleBinding},
};
use kube::{
    Api, Client,
    api::{DeleteParams, ListParams},
};

use crate::Extermination;

pub struct Exterminator {
    pub client: Client,
    pub settings: Extermination,
}

impl Exterminator {
    pub fn new(client: Client, settings: Extermination) -> Self {
        Self { client, settings }
    }

    pub async fn exterminate(&self, pod_name: String) -> Result<()> {
        let pods: Api<Pod> = Api::default_namespaced(self.client.clone());

        let delete_params = DeleteParams {
            grace_period_seconds: self.settings.grace_period,
            ..Default::default()
        };

        // Ensuring that the pod still is flagged for extermination
        let pod = pods.get(&pod_name).await?;

        let flag_label = format!("{}/enable", self.settings.labels_prefix);
        let flag_value = pod
            .metadata
            .labels
            .and_then(|labels| labels.get(&flag_label).cloned())
            .wrap_err("Pod is no longer flagged for deletion")?;

        if flag_value != "true" {
            bail!("Pod is no longer flagged for deletion");
        }

        pods.delete(&pod_name, &delete_params).await?;

        // Delete related resources
        if self.settings.delete_related {
            let selector = format!("{}/pod={pod_name}", self.settings.labels_prefix);
            let list_params = ListParams {
                label_selector: Some(selector),
                ..Default::default()
            };

            let secrets: Api<Secret> = Api::default_namespaced(self.client.clone());
            secrets
                .delete_collection(&delete_params, &list_params)
                .await?;

            let config_maps: Api<ConfigMap> = Api::default_namespaced(self.client.clone());
            config_maps
                .delete_collection(&delete_params, &list_params)
                .await?;

            let service_accounts: Api<ServiceAccount> =
                Api::default_namespaced(self.client.clone());
            service_accounts
                .delete_collection(&delete_params, &list_params)
                .await?;

            let roles: Api<Role> = Api::default_namespaced(self.client.clone());
            roles
                .delete_collection(&delete_params, &list_params)
                .await?;

            let role_bindings: Api<RoleBinding> = Api::default_namespaced(self.client.clone());
            role_bindings
                .delete_collection(&delete_params, &list_params)
                .await?;
        }

        Ok(())
    }
}
