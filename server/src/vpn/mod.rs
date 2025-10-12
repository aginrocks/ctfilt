pub mod headscale;
pub mod models;

use async_trait::async_trait;
use color_eyre::Result;
use k8s_openapi::api::core::v1::Container;
use tokio::sync::broadcast::Receiver;

use crate::vpn::models::VpnEvent;

/// Core functionality including provisioning and sidecar creation
#[async_trait]
pub trait VpnCore: Send + Sync {
    /// Creates a sidecar container that will be injected into the challenge pod
    fn get_sidecar(&self, secret_name: &str) -> Container;

    /// Generates a pre-auth key or similar that will be used by the challenge pod to authenticate to the VPN
    async fn generate_key(&self, subject: &str) -> Result<String>;
}

/// Retriving device information and updating it in realtime
#[async_trait]
pub trait VpnDevices: Send + Sync {
    /// Starts watching for device changes and updates the internal state accordingly. Should be run in a separate task.
    async fn watch(&self) -> Result<()>;

    fn subscribe(&self) -> Receiver<VpnEvent>;
}

/// Creates a global VPN provider
pub trait Vpn: VpnCore + VpnDevices {}
