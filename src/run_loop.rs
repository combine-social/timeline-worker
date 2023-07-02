use std::{env, sync::Arc, time::Duration};

use futures_util::StreamExt;
use tokio::sync::Mutex;

use crate::{
    context, home, notification, prepare,
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

fn fetch_contexts_for_tokens_loop(db: Arc<Mutex<ConnectionPool>>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let db = db.lock().await;
        if let Ok(mut connection) = repository::connect(&db).await {
            loop {
                let mut tokens = tokens::find_by_worker_id(&mut connection, worker_id());
                // TODO: rewrite to spawn a thread for each while...
                // TODO: join all handles and then sleep
                while let Some(token) = tokens.next().await {
                    println!("Got: {:?}", token);
                    _ = context::fetch_next_context(&token).await;
                }
                tokio::time::sleep(process_interval()).await;
            }
        }
    })
}

async fn queue_statuses_for_timelines(
    db: Arc<Mutex<ConnectionPool>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let db = db.lock().await;
        if let Ok(mut connection) = repository::connect(&db).await {
            loop {
                let mut tokens = tokens::find_by_worker_id(&mut connection, worker_id());
                while let Some(token) = tokens.next().await {
                    println!("Got: {:?}", token);
                    let queue_name = &token.username;
                    _ = prepare::prepare_populate_queue(queue_name).await;
                    _ = home::queue_home_statuses(&token).await;
                    _ = notification::resolve_notification_account_statuses(&token).await;
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

pub async fn perform_loop(db: ConnectionPool) {
    let db = Arc::new(Mutex::new(db));
    _ = tokio::join!(
        fetch_contexts_for_tokens_loop(db.clone()),
        queue_statuses_for_timelines(db.clone())
    );
}
