pub mod headscale;

use async_trait::async_trait;
use color_eyre::Result;
use k8s_openapi::api::core::v1::Container;

/// Core functionality including provisioning and sidecar creation
#[async_trait]
pub trait VpnCore: Send + Sync {
    /// Creates a sidecar container that will be injected into the challenge pod
    fn get_sidecar(&self, secret_name: &str) -> Container;

    /// Generates a pre-auth key or similar that will be used by the challenge pod to authenticate to the VPN
    async fn generate_key(&self, subject: &str) -> Result<String>;
}

/// Creates a global VPN provider
pub trait Vpn: VpnCore {}
