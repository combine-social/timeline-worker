use std::time::Duration;

use megalodon::{error::Kind, Megalodon, SNS};

use crate::{repository::tokens::Token, strerr::here};

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
    let result = client
        .verify_account_credentials()
        .await
        .and_then(|response| {
            if response.json().id.is_empty() {
                Err(megalodon::error::Error::new_own(
                    "Missing client id".to_string(),
                    Kind::ParseError,
                    Some(format!(
                        "{}/api/v1/accounts/verify_credentials",
                        &token.registration.instance_url
                    )),
                    None,
                ))
            } else {
                Ok(())
            }
        });
    if let Err(error) = &result {
        match error {
            megalodon::error::Error::OwnError(ref own_err) => {
                warn!(
                    "Rate limit exceeded for {}, sleeping 5 minutes",
                    &token.username
                );
                if own_err.status == Some(429) {
                    tokio::time::sleep(Duration::from_secs(300)).await;
                }
            }
            _ => warn!("has_verified_authenticated_client error: {:?}", error),
        }
    }
    result.map_err(|err| here!(err))
}
