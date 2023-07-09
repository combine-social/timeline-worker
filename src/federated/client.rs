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

pub async fn has_verified_authenticated_client(token: &Token) -> Result<(), String> {
    let client = authenticated_client(token);
    client
        .verify_account_credentials()
        .await
        .map_err(|err| err.to_string())
        .and_then(|response| {
            if response.json().id.is_empty() {
                Err("Missing client id".to_owned())
            } else {
                Ok(())
            }
        })
}
