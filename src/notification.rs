use std::env;

use chrono::Utc;
use megalodon::entities::Account;
use url::Url;

use crate::{
    cache::StatusCacheMetaData,
    federated::{self},
    repository::tokens::Token,
    send,
    strerr::here,
};

fn max_timeline_count() -> usize {
    env::var("MAX_NOTIFICATION_NEW_ACCOUNTS")
        .unwrap_or("25".to_owned())
        .parse::<usize>()
        .unwrap_or(25)
}

fn max_account_prefetch() -> usize {
    env::var("MAX_ACCOUNT_PREFETCH")
        .unwrap_or("10".to_owned())
        .parse::<usize>()
        .unwrap_or(10)
}

pub fn acct(account: &Account) -> Result<String, String> {
    if account.acct.contains('@') {
        return Ok(account.acct.clone());
    }
    if let Some(host) = Url::parse(&account.url)
        .map(|url| url.host_str().map(|s| s.to_owned()))
        .map_err(|err| here!(err))?
    {
        Ok(format!("{}@{}", account.acct, host))
    } else {
        Err("Missing host name".to_owned())
    }
}

async fn get_notification_accounts(token: &Token) -> Result<Vec<String>, String> {
    let mut max_id: Option<String> = None;
    let mut count = 0;
    let mut accounts: Vec<String> = vec![];
    loop {
        info!(
            "get_notification_timeline_page for {:?}, {:?}",
            &token.username, &max_id
        );
        let page = federated::get_notification_timeline_page(token, max_id.clone())
            .await
            .map_err(|err| {
                error!("failed get_notification_timeline_page with: {:?}", &err);
                err
            })?;
        max_id = page.max_id.clone();
        for notif in page.items.iter() {
            info!("notification: {:?}", &notif);
            let acct = acct(&notif.account)?;
            if !accounts.contains(&acct) {
                count += 1;
                accounts.push(acct);
                if count >= max_timeline_count() {
                    break;
                }
            }
        }
        if page.items.is_empty() || max_id.is_none() || count >= max_timeline_count() {
            info!("end of page list");
            break;
        }
    }
    Ok(accounts)
}

pub async fn schedule_notification_account_statuses(token: &Token) -> Result<(), String> {
    let own_instance = &token.registration.instance_url;
    let accounts = get_notification_accounts(token).await?;
    for acct in accounts {
        if !federated::is_following(token, &acct).await? {
            if let Ok(urls) =
                federated::get_remote_account_status_urls(&acct, max_account_prefetch()).await
            {
                for (index, url) in urls.into_iter().enumerate() {
                    if let Some(status_id) = url.split('/').last() {
                        _ = send::send_if_needed(
                            token,
                            own_instance,
                            &url,
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
    }
    Ok(())
}
