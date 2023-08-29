use chrono::Utc;
use megalodon::entities::Status;

use crate::{
    cache::StatusCacheMetaData,
    federated::{self, Page},
    queue_statuses,
    repository::tokens::Token,
    send,
};

async fn schedule_page_mentioned_account_statuses(token: Token, page: Page<Status>) {
    for status in page.items.iter() {
        _ = schedule_mentioned_account_statuses(&token, status)
            .await
            .map_err(|err| {
                error!("resolve_mentioned_account_statuses error: {:?}", err);
                err
            });
    }
}

async fn set_do_not_resolve_home_page(token: &Token, page: &Page<Status>) -> Result<(), String> {
    for status in page.items.iter() {
        let status = queue_statuses::status_or_reblog(status);
        if let Some(url) = status.url {
            federated::set_do_not_resolve(token, &url).await?;
        }
    }
    Ok(())
}

pub async fn queue_home_statuses(token: &Token) -> Result<(), String> {
    queue_statuses::queue_statuses(token, |max_id| async move {
        let page = federated::get_home_timeline_page(token, max_id).await?;
        set_do_not_resolve_home_page(token, &page).await?;
        info!("page has {:?} statuses", page.items.len());
        let token = token.to_owned();
        let copy = page.clone();
        #[cfg(not(test))]
        tokio::spawn(async move {
            schedule_page_mentioned_account_statuses(token, copy).await;
        });
        #[cfg(test)]
        schedule_page_mentioned_account_statuses(token, copy).await;
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

async fn schedule_mentioned_account_statuses(token: &Token, status: &Status) -> Result<(), String> {
    let own_instance = &token.registration.instance_url;
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
            for (index, url) in urls.iter().enumerate() {
                if let Some(status_id) = url.split('/').last() {
                    _ = send::send_if_needed(
                        token,
                        own_instance,
                        url,
                        &status_id.to_string(),
                        &StatusCacheMetaData {
                            original: url.clone(),
                            created_at: Utc::now(),
                            index: index as i32,
                            level: 1,
                        },
                    )
                    .await;
                }
            }
        }
    }
    Ok(())
}
