use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Extermination {
    pub delete_related: bool,
    pub grace_period: Option<u32>,
    pub labels_prefix: String,
}

impl Default for Extermination {
    fn default() -> Self {
        Self {
            delete_related: true,
            grace_period: Some(30),
            labels_prefix: "pod-assassin.agin.rocks".to_string(),
        }
    }
}
