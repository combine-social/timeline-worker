use std::{env, sync::Arc};

use megalodon::entities::Account;

use crate::{
    cache::Cache,
    federated::{
        self,
        throttle::{self, Throttle},
    },
    queue_statuses,
    repository::tokens::Token,
};

fn max_timeline_count() -> i32 {
    env::var("MAX_NOTIFICATION_NEW_ACCOUNTS")
        .unwrap_or("25".to_owned())
        .parse::<i32>()
        .unwrap_or(25)
}

pub async fn queue_notification_statuses(
    token: &Token,
    cache: &mut Cache,
    throttle: &mut Throttle,
) -> Result<(), String> {
    let mut max_id: Option<String> = None;
    let mut count = 0;
    let mut accounts: Vec<Account> = vec![];
    loop {
        let page =
            throttle::throttled(throttle, &token.registration.instance_url, None, || async {
                federated::get_notification_timeline_page(token, max_id.clone()).await
            })
            .await?;
        max_id = page.max_id.clone();
        for (i, notif) in page.items.iter().enumerate() {
            if !accounts
                .iter()
                .map(|a| a.id.clone())
                .collect::<Vec<String>>()
                .contains(&notif.account.id)
            {
                count += 1;
                accounts.push(notif.account.clone());
                if count >= max_timeline_count() {
                    break;
                }
            }
        }
        if page.items.len() == 0 || max_id.is_none() || count >= max_timeline_count() {
            break;
        }
    }
    for account in accounts {
        let account_url = Arc::new(account.url.clone());
        let account_id = Arc::new(account.id.clone());
        queue_statuses::queue_statuses(token, cache, throttle, |max_id| async {
            let account_url = account_url.clone();
            let account_id = account_id.clone();
            federated::get_account_timeline_page(
                account_id.to_string(),
                account_url.to_string(),
                max_id,
            )
            .await
        })
        .await?
    }
    Ok(())
}
