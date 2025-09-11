use std::sync::LazyLock;

use dashmap::DashMap;
use tokio::task::JoinHandle;

pub struct TimerData {
    pub handle: JoinHandle<()>,
    pub raw_value: u64,
}

pub static TIMERS: LazyLock<DashMap<String, TimerData>> = LazyLock::new(DashMap::new);
