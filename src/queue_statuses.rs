use std::env;

use chrono::{Duration, Utc};
use futures_util::Future;
use megalodon::entities::Status;
use url::Url;

use crate::{
    cache::StatusCacheMetaData,
    federated::{OriginId, Page},
    repository::tokens::Token,
    send,
};

fn since() -> Duration {
    Duration::from_std(std::time::Duration::from_secs(
        env::var("MAX_STATUS_AGE")
            .unwrap_or("86400".to_owned())
            .parse::<u64>()
            .unwrap_or(60 * 60 * 24),
    ))
    .unwrap()
}

fn max_timeline_count() -> i32 {
    env::var("MAX_STATUS_COUNT")
        .unwrap_or("75".to_owned())
        .parse::<i32>()
        .unwrap_or(75)
}

fn host(status: &Status) -> Option<String> {
    status
        .url
        .clone()
        .and_then(|url| Url::parse(&url).ok())
        .and_then(|url| url.host_str().map(|s| s.to_owned()))
}

pub fn status_or_reblog(status: &Status) -> Status {
    if status.reblog.is_some() {
        *status.reblog.to_owned().unwrap()
    } else {
        status.to_owned()
    }
}

pub async fn queue_statuses<F>(
    token: &Token,
    pager: impl Fn(Option<String>) -> F,
) -> Result<(), String>
where
    F: Future<Output = Result<Page<Status>, String>>,
{
    let mut max_id: Option<String> = None;
    let mut count = 0;
    loop {
        let page = pager(max_id.clone()).await.map_err(|err| {
            error!("Pager error: {:?}", err);
            err
        })?;
        max_id = page.max_id.clone();
        for (i, s) in page.items.iter().enumerate() {
            let created_at = s.created_at;
            let status = status_or_reblog(s);
            if status.url.clone().is_none() {
                warn!("No url for status: {:?}", &status.id);
                continue;
            }
            let now = Utc::now();
            let age = now.signed_duration_since(created_at);
            count += 1;
            if age > since() || count >= max_timeline_count() {
                info!(
                    "returning because age is {:?} or count is {:?}",
                    &age, &count
                );
                return Ok(());
            }
            if let Some(host) = host(&status) {
                match status.origin_id().await {
                    Ok(id) => {
                        _ = send::send_if_needed(
                            token,
                            &host.clone(),
                            &status.url.clone().unwrap(),
                            &id,
                            &StatusCacheMetaData {
                                original: status.url.clone().unwrap(),
                                created_at: status.created_at,
                                index: i as i32,
                                level: 1,
                            },
                        )
                        .await;
                    }
                    Err(err) => {
                        warn!("Could not get origin id for {}: {}", status.id, err);
                    }
                }
            } else {
                warn!("No host for {:?}", &status);
            }
        }
        if page.items.is_empty() || max_id.is_none() {
            info!("Page size: {:?}, max_id: {:?}", page.items.len(), &max_id);
            return Ok(());
        }
    }
}
