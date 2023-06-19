use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextRequest {
    pub status_url: String,
    pub instance_url: String,
}
