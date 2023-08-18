use std::{cell::Cell, collections::HashMap, sync::Arc, time::Duration};

use futures_util::Future;
use once_cell::sync::Lazy;
use tokio::{
    sync::{Mutex, RwLock},
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

type AccessTimeMap = Arc<RwLock<HashMap<String, Arc<Mutex<Cell<Instant>>>>>>;

/// Return a singleton hashmap of instances and access times
fn global() -> AccessTimeMap {
    static INSTANCE: Lazy<AccessTimeMap> = Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));
    INSTANCE.clone()
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
    info!("attempting to acquire lock for {}...", key);
    let mutex = get_mutex(key).await;
    let cell = mutex.lock().await;
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

/// Acquire a read lock, get cloned arc'ed cell mutex, and release lock
async fn get_mutex(key: &String) -> Arc<Mutex<Cell<Instant>>> {
    ensure_instant(key).await; // ensure that key exists
    debug!("get_mutex() attempting to acquire read lock for hashmap");
    let lock = global();
    let access_times = lock.read().await;
    let mutex = access_times.get(key).unwrap(); // unwrap is safe, because key exists
    mutex.to_owned()
}

async fn ensure_instant(key: &String) {
    if !has_key(key).await {
        create_key(key).await;
    }
}

/// Acquire a read lock, test for key existance, and release lock
async fn has_key(key: &String) -> bool {
    debug!("has_key() attempting to acquire read lock for hashmap");
    let lock = global();
    let access_times = lock.read().await;
    access_times.contains_key(key)
}

/// Acquire a write lock, create of distant_past for key, and release lock
async fn create_key(key: &String) {
    debug!("create_key() attempting to acquire write lock for hashmap...");
    let lock = global();
    let mut access_times = lock.write().await;
    debug!("inserting distant_past() for {}...", key);
    access_times.insert(
        key.to_owned(),
        Arc::new(Mutex::new(Cell::new(distant_past()))),
    );
}
