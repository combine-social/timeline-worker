use std::{collections::HashMap, time::Duration};

use futures_util::Future;
use tokio::{
    sync::Mutex,
    time::{self, Instant},
};

pub struct Throttle {
    locks: HashMap<String, Mutex<Instant>>,
}

pub fn initialize() -> Throttle {
    Throttle {
        locks: HashMap::new(),
    }
}

/// Roughly 30 years ago.
/// API does not provide a way to obtain min `Instant`
/// or convert specific date in the past to instant.
/// 1000 years overflows on macOS, 100 years overflows on FreeBSD.
fn distant_past() -> Instant {
    Instant::now() - Duration::from_secs(86400 * 365 * 30)
}

fn mutex<'a>(throttle: &'a mut Throttle, key: &String) -> &'a mut Mutex<Instant> {
    throttle
        .locks
        .entry(key.to_string())
        .or_insert(Mutex::new(distant_past()))
}

/// Perform a task at an instance.
///
/// To ensure that rate limits are not hit, request rates are
/// throttled to one request to a given instance 30 times
/// per minute - just under the rate limit.
///
/// Rate limits are set to 300 requests per 5 minutes, meaning
/// maximum one request per second (per ip and per user).
///
/// Setting this to 30 requests per minute keeps it just under the limit.
pub async fn throttled<F>(
    throttle: &mut Throttle,
    key: &String,
    requests_per_minute: Option<i32>,
    func: impl FnOnce() -> F,
) where
    F: Future,
{
    let max_delay = 60.0 / requests_per_minute.unwrap_or(30) as f64;
    let mutex = mutex(throttle, key);
    let mut instant = mutex.lock().await;
    let duration = Instant::now().duration_since(*instant);
    let delay = max_delay - duration.as_secs_f64();
    if delay > 0.0 {
        time::sleep(Duration::from_secs_f64(delay)).await;
    }
    func().await;
    *instant = Instant::now();
}
