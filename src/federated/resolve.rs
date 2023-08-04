use megalodon::megalodon::{SearchInputOptions, SearchType};

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

/// Resolve a remote status on the instance which the token belongs to.
/// This runs non-blocking on a separate green thread.
pub async fn resolve(token: &Token, status_url: &String) {
    let token = token.to_owned();
    let status_url = status_url.to_owned();
    tokio::spawn(async {
        perform_resolve(token, status_url).await;
    });
}

async fn perform_resolve(token: Token, status_url: String) {
    let key = &token.registration.instance_url;
    throttle::throttled(key, None, || async {
        _ = client::authenticated_client(&token)
            .search(
                status_url.to_owned(),
                &SearchType::Statuses,
                search_options(),
            )
            .await;
    })
    .await;
}
