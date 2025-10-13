use serde::Serialize;
use ts_rs::TS;

use crate::{orchestrator::RunningChallenge, vpn::models::VpnDevice};

#[derive(Serialize, TS)]
#[ts(export)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ServerMessage {
    ChallengesUpdate(ChallengesUpdate),
    VpnState(VpnState), // Solve(Solve),
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ChallengesUpdate {
    pub challenges: Vec<RunningChallenge>,
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct VpnState {
    pub devices: Vec<VpnDevice>,
}

// #[derive(Serialize, TS)]
// #[ts(export)]
// pub struct Solve {
//     pub challenge_slug: String,
// }
