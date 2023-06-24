use std::env;

use chrono::{Duration, Utc};
use megalodon::entities::Status;
use url::Url;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    conditional_queue,
    federated::{
        self,
        throttle::{self, Throttle},
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

pub async fn queue_home_statuses(
    token: &Token,
    cache: &mut Cache,
    throttle: &mut Throttle,
) -> Result<(), String> {
    let mut max_id: Option<String> = None;
    let mut count = 0;
    loop {
        let response =
            throttle::throttled(throttle, &token.registration.instance_url, None, || async {
                federated::get_home_timeline_page(token, &max_id.clone()).await
            })
            .await?;
        max_id = federated::max_id(&response);
        for (i, s) in response.json().iter().enumerate() {
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
                    &cache::status_key(
                        &token.registration.instance_url,
                        &status.url.clone().unwrap(),
                    ),
                    &ContextRequest {
                        instance_url: host.to_string(),
                        status_id: status.id.clone(),
                        status_url: status.url.clone().unwrap(),
                    },
                    &StatusCacheMetaData {
                        original: status.url.clone().unwrap(),
                        created_at: status.created_at,
                        index: 0,
                        level: i as i32,
                    },
                )
                .await;
            }
        }
    }
}
