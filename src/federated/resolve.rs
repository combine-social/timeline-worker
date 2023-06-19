use megalodon::entities::Results;
use megalodon::megalodon::{SearchInputOptions, SearchType};
use megalodon::response::Response;

use crate::repository::tokens::Token;

use super::client;
use super::throttle::{self, Throttle};

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

fn unwrap_status_url(
    result: Result<Response<Results>, megalodon::error::Error>,
) -> Result<Option<String>, megalodon::error::Error> {
    if let Ok(response) = result {
        let status = response.json.statuses.first();
        if let Some(status) = status {
            Ok(status.url.clone())
        } else {
            Ok(None)
        }
    } else {
        Err(result.err().unwrap())
    }
}

/// Resolve a remote status on the instance whivch the token belongs to.
/// Returns optional status url on success.
pub async fn resolve(
    token: &Token,
    status_url: &String,
    throttle: &mut Throttle,
) -> Result<Option<String>, megalodon::error::Error> {
    let key = &token.registration.instance_url;
    throttle::throttled(throttle, key, None, || async {
        unwrap_status_url(
            client::authenticated_client(token)
                .search(
                    status_url.to_owned(),
                    &SearchType::Statuses,
                    search_options(),
                )
                .await,
        )
    })
    .await
}
