use std::sync::LazyLock;

use dashmap::{DashMap, DashSet};
use tokio::task::JoinHandle;

pub struct TimerData {
    pub handle: JoinHandle<()>,
    pub raw_value: u64,
}

pub static TIMERS: LazyLock<DashMap<String, TimerData>> = LazyLock::new(DashMap::new);

// TODO: fix the race condition if timer changes values while deleting
pub static LOCKS: LazyLock<DashSet<String>> = LazyLock::new(DashSet::new);
