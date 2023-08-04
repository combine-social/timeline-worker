use serde::Deserialize;
use std::clone::Clone;

#[derive(Clone, Deserialize, Debug)]
pub struct Registration {
    pub id: i32,
    pub instance_url: String,
    pub registration_id: Option<String>,
    pub name: Option<String>,
    pub website: Option<String>,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub vapid_key: Option<String>,
    pub nonce: String,
}
