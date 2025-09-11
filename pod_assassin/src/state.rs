use std::{collections::HashSet, sync::LazyLock};

use dashmap::{DashMap, DashSet};
use tokio::{sync::Mutex, task::JoinHandle};

pub struct TimerData {
    pub handle: JoinHandle<()>,
    pub raw_value: u64,
}

pub static TIMERS: LazyLock<DashMap<String, TimerData>> = LazyLock::new(DashMap::new);

pub static LOCKS: LazyLock<DashSet<String>> = LazyLock::new(DashSet::new);
