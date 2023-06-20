use std::env;

use futures_util::StreamExt;

use crate::{
    cache::Cache,
    context,
    federated::throttle::Throttle,
    queue::Connection,
    repository::{self, tokens, ConnectionPool},
};

fn worker_id() -> i32 {
    env::var("WORKER_ID")
        .unwrap_or("1".to_owned())
        .parse::<i32>()
        .unwrap_or(1)
}

async fn fetch_contexts_for_tokens(
    db: &ConnectionPool,
    cache: &mut Cache,
    queue: &Connection,
    throttle: &mut Throttle,
) {
    if let Ok(mut connection) = repository::connect(db).await {
        let mut tokens = tokens::find_by_worker_id(&mut connection, worker_id());
        while let Some(token) = tokens.next().await {
            println!("Got: {:?}", token);
            let _ = context::fetch_next_context(&token, cache, queue, throttle).await;
        }
    }
}

pub async fn perform_loop(
    db: &ConnectionPool,
    cache: &mut Cache,
    queue: &mut Connection,
    throttle: &mut Throttle,
) -> ! {
    loop {
        fetch_contexts_for_tokens(db, cache, queue, throttle).await;
    }
}
