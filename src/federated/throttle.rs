use std::{collections::HashMap, time::Duration};

use futures_util::Future;
use tokio::{
    sync::Mutex,
    time::{self, Instant},
};

pub struct Throttle {
    locks: Option<Mutex<HashMap<String, Instant>>>,
}

#[allow(non_upper_case_globals)]
static mut throttle: Throttle = Throttle { locks: None };

pub fn initialize() {
    unsafe {
        let locks = HashMap::new();
        throttle.locks = Some(Mutex::new(locks));
    }
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
    unsafe {
        if let Some(locks) = &throttle.locks {
            let locks = locks.lock().await;
            if let Some(instant) = locks.get(key) {
                let max_delay = 60.0 / requests_per_minute.unwrap_or(default_rpm()) as f64;
                let duration = Instant::now().duration_since(instant.to_owned());
                let delay = max_delay - duration.as_secs_f64();
                if delay > 0.0 {
                    time::sleep(Duration::from_secs_f64(delay)).await;
                }
            }
        }
    }
    let result = func().await;
    unsafe {
        if let Some(locks) = &throttle.locks {
            let mut locks = locks.lock().await;
            locks.insert(key.to_owned(), Instant::now());
        }
    }
    result
}
