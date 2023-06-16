use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuedStatus {
    pub instance_url: String,
    pub status_url: String,
}
