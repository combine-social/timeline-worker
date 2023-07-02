use std::env;

use crate::{
    cache::{self, Cache},
    federated::{
        self,
        throttle::{self},
    },
    repository::tokens::Token,
};

fn max_timeline_count() -> usize {
    env::var("MAX_NOTIFICATION_NEW_ACCOUNTS")
        .unwrap_or("25".to_owned())
        .parse::<usize>()
        .unwrap_or(25)
}

async fn get_notification_accounts(token: &Token) -> Result<Vec<String>, String> {
    let mut max_id: Option<String> = None;
    let mut count = 0;
    let mut accounts: Vec<String> = vec![];
    loop {
        let page = throttle::throttled(&token.registration.instance_url, None, || async {
            federated::get_notification_timeline_page(token, max_id.clone()).await
        })
        .await?;
        max_id = page.max_id.clone();
        for notif in page.items.iter() {
            if !accounts.contains(&notif.account.acct) {
                count += 1;
                accounts.push(notif.account.acct.clone());
                if count >= max_timeline_count() {
                    break;
                }
            }
        }
        if page.items.is_empty() || max_id.is_none() || count >= max_timeline_count() {
            break;
        }
    }
    Ok(accounts)
}

pub async fn resolve_notification_account_statuses(token: &Token) -> Result<(), String> {
    let accounts = get_notification_accounts(token).await?;
    for acct in accounts {
        // TODO: filter out followed accounts
        let urls = federated::get_remote_account_status_urls(&acct, max_timeline_count()).await?;
        for url in urls {
            federated::resolve(token, &url).await?;
            todo!();
        }
    }
    Ok(())
}
