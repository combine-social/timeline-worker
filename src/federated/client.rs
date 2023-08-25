use std::time::Duration;

use crate::repository::registrations;
use megalodon::{error::Kind, Megalodon, SNS};

use crate::{repository::tokens::Token, strerr::here};

use super::detect;

fn registration_sns(token: &Token) -> SNS {
    token
        .registration
        .sns
        .clone()
        .unwrap_or(registrations::SNS::Mastodon)
        .into()
}

async fn instance_sns(instance_url: &str) -> SNS {
    detect::detect_sns(instance_url)
        .await
        .unwrap_or_else(|err| {
            warn!(
                "Detecting instance software on {} failed, assuming Mastodon and 🤞: {}",
                instance_url, err
            );
            SNS::Mastodon
        }) // Assume mastodon on failure - this will likely fail at a later stage
}

pub fn authenticated_client(token: &Token) -> Box<dyn Megalodon + Send + Sync> {
    megalodon::generator(
        registration_sns(token),
        format!("https://{}", token.registration.instance_url),
        Some(token.access_token.clone()),
        None,
    )
}

pub async fn anonymous_client(url: &str) -> Box<dyn Megalodon + Send + Sync> {
    megalodon::generator(
        instance_sns(url).await,
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
