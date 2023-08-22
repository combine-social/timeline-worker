use megalodon::entities::Status;

use crate::{
    federated::{self, Page},
    queue_statuses,
    repository::tokens::Token,
};

async fn resolve_page_mentioned_account_statuses(token: Token, page: Page<Status>) {
    for status in page.items.iter() {
        _ = resolve_mentioned_account_statuses(&token, status)
            .await
            .map_err(|err| {
                error!("resolve_mentioned_account_statuses error: {:?}", err);
                err
            });
    }
}

pub async fn queue_home_statuses(token: &Token) -> Result<(), String> {
    queue_statuses::queue_statuses(token, |max_id| async move {
        let page = federated::get_home_timeline_page(token, max_id).await?;
        info!("page has {:?} statuses", page.items.len());
        let token = token.to_owned();
        let copy = page.clone();
        tokio::spawn(async move {
            resolve_page_mentioned_account_statuses(token, copy).await;
        });
        Ok(page)
    })
    .await
}

fn get_mentions(status: &Status) -> Vec<String> {
    status
        .mentions
        .iter()
        .map(|mention| mention.acct.clone())
        .collect()
}

async fn resolve_mentioned_account_statuses(token: &Token, status: &Status) -> Result<(), String> {
    let accounts = get_mentions(status);
    for acct in accounts {
        if !acct.contains('@') {
            warn!("Missing hostname in acct: {}", acct);
            continue;
        }
        if !federated::is_following(token, &acct).await? {
            let urls = federated::get_remote_account_status_urls(&acct, 25)
                .await
                .unwrap_or_else(|err| {
                    error!(
                        "Error getting status urls for account: {}: {:?}",
                        &acct, err
                    );
                    vec![]
                });
            for url in urls {
                federated::resolve(token, &url).await;
            }
        }
    }
    Ok(())
}
