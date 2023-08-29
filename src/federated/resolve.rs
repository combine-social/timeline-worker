use megalodon::megalodon::{SearchInputOptions, SearchType};

use crate::cache;
use crate::repository::tokens::Token;

use super::client;
use super::throttle::{self};

fn search_options() -> Option<&'static SearchInputOptions> {
    Some(&SearchInputOptions {
        r#type: Some(SearchType::Statuses),
        limit: Some(1),
        max_id: None,
        min_id: None,
        resolve: Some(true),
        offset: None,
        following: None,
        account_id: None,
        exclude_unreviewed: None,
    })
}

/// Set resolve key to avoid multiple resolves of same url.
pub async fn set_do_not_resolve(token: &Token, status_url: &str) -> Result<(), String> {
    let mut cache = cache::connect().await?;
    let key = cache::resolve_key(&token.registration.instance_url, &status_url.to_string());
    cache::set(&mut cache, &key, &true, Some(24 * 60 * 60)).await?;
    Ok(())
}

/// Returns true if the key wasn't already set (if it hasn't been resolved yet).
pub async fn should_resolve(token: &Token, status_url: &str) -> Result<bool, String> {
    let mut cache = cache::connect().await?;
    let key = cache::resolve_key(&token.registration.instance_url, &status_url.to_string());
    let has_key = cache::has(&mut cache, &key).await?;
    Ok(!has_key)
}

/// Resolve a remote status on the instance which the token belongs to.
pub async fn resolve(token: &Token, status_url: &String) {
    if should_resolve(token, status_url)
        .await
        .is_ok_and(|should| !should)
    {
        info!("Status {} is already resolved, skipping", status_url);
        return;
    }
    if let Err(err) = set_do_not_resolve(token, status_url).await {
        error!("Could not set_do_not_resolve: {}", err);
    }
    let key = &cache::user_key(&token.username);
    info!("throttled call to search ({})", &status_url);
    throttle::throttled(key, None, || async {
        _ = client::authenticated_client(token)
            .search(status_url.to_owned(), search_options())
            .await;
    })
    .await;
}
