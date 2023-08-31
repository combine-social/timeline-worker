use crate::repository::registrations::Registration;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::clone::Clone;

pub trait PingAt {
    fn latest_ping(&self) -> DateTime<Utc>;
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Token {
    pub id: i32,
    pub username: String,
    pub access_token: String,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub created_at: Option<i32>,
    pub fail_count: Option<i32>,
    pub registration: Registration,
    pub worker_id: i32,
    pub ping_at: Option<i32>,
}

impl PingAt for Token {
    fn latest_ping(&self) -> DateTime<Utc> {
        let default = Utc.timestamp_opt(0, 0).single().unwrap();
        Utc.timestamp_opt(self.ping_at.unwrap_or(0) as i64, 0)
            .single()
            .unwrap_or(default)
    }
}
