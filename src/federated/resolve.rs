use megalodon::entities::{Results, Status};
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

fn unwrap_status(
    result: Result<Response<Results>, megalodon::error::Error>,
) -> Result<Option<Status>, megalodon::error::Error> {
    if let Ok(response) = result {
        let status = response.json.statuses.first();
        if let Some(status) = status {
            Ok(Some(status.to_owned()))
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
) -> Result<Option<Status>, String> {
    let key = &token.registration.instance_url;
    throttle::throttled(throttle, key, None, || async {
        unwrap_status(
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
    .map_err(|err| err.to_string())
}
