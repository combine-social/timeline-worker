use std::{env, sync::Arc};

use futures_util::Future;
use once_cell::sync::Lazy;
use rslock::LockManager;

fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or("redis://localhost".to_owned())
}

/// Return a singleton lock manager
fn global() -> Arc<LockManager> {
    static MANAGER: Lazy<Arc<LockManager>> =
        Lazy::new(|| Arc::new(LockManager::new(vec![redis_url()])));
    MANAGER.clone()
}

fn default_rpm() -> i32 {
    #[cfg(test)]
    return 10000;
    #[cfg(not(test))]
    30
}

/// Return time to live in milis
fn ttl(rpm: Option<i32>) -> usize {
    60000 / rpm.unwrap_or(default_rpm()) as usize
}

fn lock_name(key: &String) -> Vec<u8> {
    format!("{}:mutex", key).as_bytes().to_owned()
}

/// Perform a task at an instance.
///
/// To ensure that rate limits are not hit, request rates are
/// throttled to one request to a given instance 30 times
/// per minute - just under the rate limit.
///
/// Per-IP rate limit is set to 7500 reqs per 5 minutes.
///
/// Per user rate limits are set to 300 requests per 5 minutes, meaning
/// maximum one request per second (per ip and per user).
///
/// Setting this to 30 requests per minute keeps it just under the limit.
pub async fn throttled<F, R>(key: &String, requests_per_minute: Option<i32>, func: F) -> R::Output
where
    F: Fn() -> R,
    R: Future,
{
    info!("acquiring lock for {}:mutex...", key);
    let manager = global();
    let name = lock_name(key);
    _ = manager
        .lock(&name, ttl(requests_per_minute))
        .await
        .map_err(|err| {
            error!(
                "@{}#{}: Failed locking {}:mutex: {:?}",
                file!(),
                line!(),
                key,
                err
            )
        });
    func().await
}
