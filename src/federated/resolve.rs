use megalodon::megalodon::{SearchInputOptions, SearchType};
use url::Url;

use crate::cache;
use crate::repository::tokens::Token;

use super::client;
use super::throttle::{self};

fn search_options() -> Option<&'static SearchInputOptions> {
    Some(&SearchInputOptions {
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

fn host(url: &str) -> Result<String, String> {
    let result = Url::parse(url);
    if let Ok(url) = result {
        if let Some(host) = url.host_str() {
            Ok(host.to_string())
        } else {
            let message = format!("Missing host in {}", url);
            error!("{}", message);
            Err(message)
        }
    } else {
        Err(result.err().unwrap().to_string())
    }
}

pub fn is_remote(token: &Token, status_url: &str) -> Result<bool, String> {
    Ok(token.registration.instance_url != host(status_url)?)
}

/// Resolve a remote status on the instance which the token belongs to.
pub async fn resolve(token: &Token, status_url: &String) {
    if !is_remote(token, status_url).is_ok_and(|remote| remote) {
        info!(
            "Status {} is local to {}, skipping",
            &status_url, &token.registration.instance_url
        );
        return;
    }
    let key = &cache::user_key(&token.username);
    info!("throttled call to search ({})", &status_url);
    throttle::throttled(key, None, || async {
        _ = client::authenticated_client(token)
            .search(
                status_url.to_owned(),
                &SearchType::Statuses,
                search_options(),
            )
            .await;
    })
    .await;
}
