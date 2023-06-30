use std::env;

use chrono::{Duration, Utc};
use futures_util::Future;
use megalodon::entities::Status;
use url::Url;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    conditional_queue,
    federated::{
        throttle::{self},
        Page,
    },
    models::ContextRequest,
    repository::tokens::Token,
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
        .unwrap_or("25".to_owned())
        .parse::<i32>()
        .unwrap_or(25)
}

fn host(status: &Status) -> Option<String> {
    status
        .url
        .clone()
        .and_then(|url| Url::parse(&url).ok())
        .and_then(|url| url.host_str().map(|s| s.to_owned()))
}

fn status_or_reblog(status: &Status) -> Status {
    if status.reblog.is_some() {
        *status.reblog.to_owned().unwrap()
    } else {
        status.to_owned()
    }
}

pub async fn queue_statuses<F>(
    token: &Token,
    cache: &mut Cache,
    pager: impl Fn(Option<String>) -> F,
) -> Result<(), String>
where
    F: Future<Output = Result<Page<Status>, String>>,
{
    let mut max_id: Option<String> = None;
    let mut count = 0;
    loop {
        let page = throttle::throttled(&token.registration.instance_url, None, || async {
            pager(max_id.clone()).await
        })
        .await?;
        max_id = page.max_id.clone();
        for (i, s) in page.items.iter().enumerate() {
            let status = status_or_reblog(s);
            if status.url.is_none() {
                continue;
            }
            let now = Utc::now();
            let age = now.signed_duration_since(status.created_at);
            count += 1;
            if age > since() || count >= max_timeline_count() {
                return Ok(());
            }
            if let Some(host) = host(&status) {
                _ = conditional_queue::send_if_not_cached(
                    cache,
                    &token.username,
                    &cache::status_key(&host.clone(), &status.url.clone().unwrap()),
                    &ContextRequest {
                        instance_url: host.clone(),
                        status_id: status.id.clone(),
                        status_url: status.url.clone().unwrap(),
                    },
                    &StatusCacheMetaData {
                        original: status.url.clone().unwrap(),
                        created_at: status.created_at,
                        index: i as i32,
                        level: 0,
                    },
                )
                .await;
            }
        }
        if page.items.is_empty() || max_id.is_none() {
            return Ok(());
        }
    }
}
