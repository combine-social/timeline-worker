use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCacheMetaData {
    pub original: String,
    pub created_at: String,
    pub index: i32,
    pub level: i32,
}
