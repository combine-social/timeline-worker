use serde::{Deserialize, Serialize};
use std::{clone::Clone, convert::From};

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
    pub sns: Option<SNS>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Deserialize, Debug, Serialize, PartialEq)]
pub enum SNS {
    #[serde(rename = "mastodon")]
    Mastodon,
    #[serde(rename = "pleroma")]
    Pleroma,
    #[serde(rename = "friendica")]
    Friendica,
}

impl From<megalodon::SNS> for SNS {
    fn from(item: megalodon::SNS) -> Self {
        match item {
            megalodon::SNS::Mastodon => SNS::Mastodon,
            megalodon::SNS::Pleroma => SNS::Pleroma,
            megalodon::SNS::Friendica => SNS::Friendica,
        }
    }
}

impl From<SNS> for megalodon::SNS {
    fn from(item: SNS) -> Self {
        match item {
            SNS::Mastodon => megalodon::SNS::Mastodon,
            SNS::Pleroma => megalodon::SNS::Pleroma,
            SNS::Friendica => megalodon::SNS::Friendica,
        }
    }
}
