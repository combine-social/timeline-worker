use crate::repository::registrations::Registration;
use serde::{Deserialize, Serialize};
use std::clone::Clone;

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
}
