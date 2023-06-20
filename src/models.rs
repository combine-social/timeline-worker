use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextRequest {
    pub instance_url: String,
    pub status_id: String,
    pub status_url: String,
}
