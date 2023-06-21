use std::{env, sync::Arc, time::Duration};

use futures_util::StreamExt;
use tokio::sync::Mutex;

use crate::{
    cache::Cache,
    context,
    federated::throttle::Throttle,
    repository::{self, tokens, ConnectionPool},
};

fn worker_id() -> i32 {
    env::var("WORKER_ID")
        .unwrap_or("1".to_owned())
        .parse::<i32>()
        .unwrap_or(1)
}

fn poll_interval() -> Duration {
    Duration::from_secs(
        env::var("POLL_INTERVAL")
            .unwrap_or("300".to_owned())
            .parse::<u64>()
            .unwrap_or(300),
    )
}

fn fetch_contexts_for_tokens_loop(
    db: Arc<Mutex<ConnectionPool>>,
    cache: Arc<Mutex<Cache>>,
    throttle: Arc<Mutex<Throttle>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let db = db.lock().await;
        let mut cache = cache.lock().await;
        let mut throttle = throttle.lock().await;
        if let Ok(mut connection) = repository::connect(&db).await {
            loop {
                let mut tokens = tokens::find_by_worker_id(&mut connection, worker_id());
                while let Some(token) = tokens.next().await {
                    println!("Got: {:?}", token);
                    _ = context::fetch_next_context(&token, &mut cache, &mut throttle).await;
                }
                println!(
                    "Waiting: {:?}s before processing tokens...",
                    poll_interval().as_secs()
                );
                tokio::time::sleep(poll_interval()).await;
            }
        }
    })
}

pub async fn perform_loop(db: ConnectionPool, cache: Cache, throttle: Throttle) {
    let db = Arc::new(Mutex::new(db));
    let cache = Arc::new(Mutex::new(cache));
    let throttle = Arc::new(Mutex::new(throttle));
    _ = fetch_contexts_for_tokens_loop(db, cache, throttle).await;
}
