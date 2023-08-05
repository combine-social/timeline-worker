use std::{collections::HashMap, time::Duration};

use futures_util::Future;
use once_cell::sync::Lazy;
use tokio::{
    sync::{Mutex, MutexGuard},
    time::{self, Instant},
};

async fn global_lock() -> MutexGuard<'static, HashMap<String, Instant>> {
    static mut INSTANCE: Lazy<Mutex<HashMap<String, Instant>>> =
        Lazy::new(|| Mutex::new(HashMap::new()));
    unsafe { INSTANCE.lock() }.await
}

fn default_rpm() -> i32 {
    #[cfg(test)]
    return 10000;
    #[cfg(not(test))]
    30
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
    wait_if_needed(key, requests_per_minute).await;
    let result = func().await;
    update_access_time(key).await;
    result
}

async fn wait_if_needed(key: &String, requests_per_minute: Option<i32>) {
    let locks = global_lock().await;
    if let Some(instant) = locks.get(key) {
        let max_delay = 60.0 / requests_per_minute.unwrap_or(default_rpm()) as f64;
        let duration = Instant::now().duration_since(instant.to_owned());
        let delay = max_delay - duration.as_secs_f64();
        if delay > 0.0 {
            info!("waiting {:.1}s on {}", delay, key);
            time::sleep(Duration::from_secs_f64(delay)).await;
        } else {
            info!("no wait needed for {}", key);
        }
    } else {
        info!("no lock found for for {}", key);
    }
}

async fn update_access_time(key: &String) {
    let mut locks = global_lock().await;
    info!("setting access time to now for {}", key);
    locks.insert(key.to_owned(), Instant::now());
}
