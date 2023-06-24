use std::{env, sync::Arc, time::Duration};

use futures_util::StreamExt;
use tokio::sync::Mutex;

use crate::{
    cache::Cache,
    context,
    federated::throttle::Throttle,
    home, prepare,
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

fn process_interval() -> Duration {
    Duration::from_secs(
        env::var("PROCESS_INTERVAL")
            .unwrap_or("2".to_owned())
            .parse::<u64>()
            .unwrap_or(2),
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
                tokio::time::sleep(process_interval()).await;
            }
        }
    })
}

async fn queue_statuses_for_timelines(
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
                    let queue_name = &token.username;
                    _ = prepare::prepare_populate_queue(&mut cache, queue_name).await;
                    _ = home::queue_home_statuses(&token, &mut cache, &mut throttle).await;
                    // todo: process notification timeline
                }
                println!(
                    "Waiting: {:?}s before processing timelines for tokens...",
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
    _ = tokio::join!(
        fetch_contexts_for_tokens_loop(db.clone(), cache.clone(), throttle.clone()),
        queue_statuses_for_timelines(db.clone(), cache.clone(), throttle.clone())
    );
}
