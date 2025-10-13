// User-facing models for VPN

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[serde(rename_all = "lowercase")]
pub enum VpnDeviceType {
    // TODO: Add challenge details
    Challenge,
    Client,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
pub struct VpnDevice {
    pub r#type: VpnDeviceType,
    pub name: String,
    pub hostname: String,
    pub ip_addresses: Vec<String>,
    pub last_seen: Option<String>,
    pub created_at: Option<String>,
    pub online: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
pub struct VpnEvent {
    pub user_subject: String,

    #[serde(flatten)]
    pub event: VpnEventCore,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
pub enum VpnEventCore {
    Connected { data: VpnDevice },
    Disconnected { data: VpnDevice },
    StateChanged { data: Vec<VpnDevice> },
}
