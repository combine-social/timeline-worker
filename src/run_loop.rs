use std::env;

use futures_util::StreamExt;
use tokio::time::{self, Duration};

use crate::{
    cache::Cache,
    queue::Connection,
    repository::{self, tokens, ConnectionPool},
};

fn worker_id() -> i32 {
    env::var("WORKER_ID")
        .unwrap_or("1".to_owned())
        .parse::<i32>()
        .unwrap_or(1)
}

async fn get_contexts_for_tokens(db: &ConnectionPool) {
    time::sleep(Duration::from_millis(60_000 / 30)).await;
    if let Ok(mut connection) = repository::connect(&db).await {
        let mut tokens = tokens::find_by_worker_id(&mut connection, worker_id());
        while let Some(token) = tokens.next().await {
            println!("Got: {:?}", token);
            // TODO: port getNextContext(token)
        }
    }
}

pub async fn perform_loop(db: &ConnectionPool, cache: &mut Cache, queue: &mut Connection) -> ! {
    loop {
        get_contexts_for_tokens(db).await;
    }
}
