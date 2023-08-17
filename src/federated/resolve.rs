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
pub async fn resolve(token: &Token, status_url: &String) {
    let key = &token.registration.instance_url;
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
