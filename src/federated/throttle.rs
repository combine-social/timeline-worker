use std::{cell::Cell, collections::HashMap, time::Duration};

use futures_util::Future;
use once_cell::sync::Lazy;
use tokio::{
    sync::Mutex,
    time::{self, Instant},
};

/// Return distant past - or rather, distant enough for no sleep to be needed
fn distant_past() -> Instant {
    Instant::now()
        .checked_sub(Duration::from_secs(
            chrono::Duration::minutes(5).num_seconds() as u64,
        ))
        .unwrap()
}

fn global() -> &'static mut Lazy<HashMap<String, Mutex<Cell<Instant>>>> {
    static mut INSTANCE: Lazy<HashMap<String, Mutex<Cell<Instant>>>> = Lazy::new(HashMap::new);
    unsafe { &mut INSTANCE }
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
    let access_times = global();
    info!("attempting to acquire lock for {}...", key);
    let cell = access_times
        .entry(key.to_owned())
        .or_insert(Mutex::new(Cell::new(distant_past())))
        .lock()
        .await;
    let instant = cell.get();
    let max_delay = 60.0 / requests_per_minute.unwrap_or(default_rpm()) as f64;
    let duration = Instant::now().duration_since(instant);
    let delay = max_delay - duration.as_secs_f64();
    if delay > 0.0 {
        info!("waiting {:.1}s on {}", delay, key);
        time::sleep(Duration::from_secs_f64(delay)).await;
    } else {
        info!("no wait needed for {}", key);
    }
    let result = func().await;
    info!("setting access time to now for {}", key);
    cell.set(Instant::now());
    result
}
