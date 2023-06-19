use chrono::{serde::ts_seconds, DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCacheMetaData {
    pub original: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    pub index: i32,
    pub level: i32,
}
