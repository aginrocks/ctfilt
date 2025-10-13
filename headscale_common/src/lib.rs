use headscale::models::V1Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodesChangedMessage {
    pub user: String,
    pub nodes: Vec<V1Node>,
}
