use crate::repository::registrations::models::Registration;

#[derive(Debug)]
pub struct Token {
    pub id: i32,
    pub username: String,
    pub access_token: String,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub created_at: Option<i32>,
    pub fail_count: Option<i32>,
    pub registration: Registration,
}
