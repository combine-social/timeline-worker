use megalodon::entities::Status;

use crate::{
    federated::{self},
    queue_statuses,
    repository::tokens::Token,
};

pub async fn queue_home_statuses(token: &Token) -> Result<(), String> {
    queue_statuses::queue_statuses(token, |max_id| async move {
        let page = federated::get_home_timeline_page(token, max_id).await?;
        for status in page.items.iter() {
            resolve_mentioned_account_statuses(token, status).await?;
        }
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
        if !federated::is_following(token, &acct).await? {
            let urls = federated::get_remote_account_status_urls(&acct, 25).await?;
            for url in urls {
                federated::resolve(token, &url).await?;
            }
        }
    }
    Ok(())
}
