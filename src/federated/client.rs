use megalodon::{Megalodon, SNS};

use crate::repository::tokens::Token;

pub fn authenticated_client(token: &Token) -> Box<dyn Megalodon + Send + Sync> {
    megalodon::generator(
        SNS::Mastodon, // TODO: update tokens table to include sns
        format!("https://{}", token.registration.instance_url),
        Some(token.access_token.clone()),
        None,
    )
}
pub fn anonymous_client(url: &str, sns: Option<SNS>) -> Box<dyn Megalodon + Send + Sync> {
    megalodon::generator(
        sns.unwrap_or(SNS::Mastodon),
        format!("https://{}", url),
        None,
        None,
    )
}
