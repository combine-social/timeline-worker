use std::{env, time::Duration};

use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::{
    context, federated, home, notification, prepare,
    repository::tokens::{PingAt, Token},
    tokens,
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

/// If requests with a token fails for more than 10 minutes
/// (1 request every 2 seconds) then assume that it has been
/// revoked and delete it.
fn max_fail_count() -> i32 {
    env::var("TOKEN_MAX_FAIL_COUNT")
        .unwrap_or("300".to_owned())
        .parse::<i32>()
        .unwrap_or(60 * 5)
}

/// Verify token
///
/// Check that a token can be used to create an authenticated client
/// and update the fail count. Delete tokens whose fail count goes
/// over the set threshold.
async fn verify_token(pool: &Pool<Postgres>, token: &Token) -> Result<(), String> {
    let result = federated::has_verified_authenticated_client(token).await;
    if result.is_ok() {
        tokens::update_token_fail_count(pool, worker_id(), token, 0).await?;
    } else {
        let fail_count = token.fail_count.unwrap_or(0) + 1;
        if fail_count > max_fail_count() {
            warn!(
                "Fail-count threshold exceeded for {}, deleting",
                &token.username
            );
            tokens::delete_token(pool, worker_id(), token).await?;
        } else {
            tokens::update_token_fail_count(pool, worker_id(), token, fail_count).await?;
        }
    }
    result
}

async fn perform_fetch_contexts(pool: &Pool<Postgres>, token: Token) {
    if verify_token(pool, &token).await.is_ok() {
        info!(
            "fetch_contexts_for_tokens_loop got token for: {:?}",
            &token.username
        );
        const TIMEOUT: i64 = 300; // 5 minute timeout
        if Utc::now()
            .signed_duration_since(token.latest_ping())
            .num_seconds()
            < TIMEOUT
        {
            return;
        }
        tokio::spawn(async move {
            _ = tokens::ping_token(worker_id(), &token).await;
            while context::fetch_next_context(&token.to_owned())
                .await
                .is_ok_and(|more| more)
            {
                info!("Fetching next context for: {}", &token.username);
                _ = tokens::ping_token(worker_id(), &token).await;
            }
        });
    } else {
        warn!("Could not verify token for {}", &token.username);
    }
}

async fn fetch_contexts_for_tokens_loop(pool: &Pool<Postgres>) {
    loop {
        if let Ok(tokens) = tokens::get_tokens(worker_id()).await {
            for token in tokens {
                perform_fetch_contexts(pool, token).await;
            }
        }
        info!(
            "Waiting: {:?}s before fetching contexts for tokens...",
            process_interval().as_secs()
        );
        tokio::time::sleep(process_interval()).await;
    }
}

async fn perform_repopulate(pool: &Pool<Postgres>, token: Token) {
    info!(
        "queue_statuses_for_timelines got token for: {}",
        &token.username
    );
    if verify_token(pool, &token).await.is_ok() {
        let queue_name = format!("v2:{}", &token.username);
        if prepare::should_populate_queue(&queue_name).await {
            _ = tokens::clear_token_ping(worker_id(), &token).await;
            if let Err(err) = prepare::prepare_populate_queue(&queue_name).await {
                error!("Error in prepare_populate_queue: {:?}", err);
            }
            if let Err(err) = home::queue_home_statuses(&token).await {
                error!("Error in queue_home_statuses: {:?}", err);
            }
            if let Err(err) = notification::schedule_notification_account_statuses(&token).await {
                error!("Error in resolve_notification_account_statuses: {:?}", err);
            }
        } else {
            info!(
                "Queue {} above threshold, postponing repopulate...",
                &token.username
            );
        }
    } else {
        warn!("Could not verify token for: {}", &token.username);
    }
}

async fn queue_statuses_for_timelines(pool: Pool<Postgres>) {
    loop {
        if let Err(err) = tokens::refresh_tokens(&pool.clone(), worker_id()).await {
            error!("Error refreshing tokens: {:?}", err);
        }
        match tokens::get_tokens(worker_id()).await {
            Ok(tokens) => {
                for token in tokens {
                    let pool = pool.clone();
                    tokio::spawn(async move {
                        perform_repopulate(&pool, token).await;
                    });
                }
            }
            Err(err) => {
                error!("Error getting tokens: {:?}", err);
            }
        }
        info!(
            "Waiting: {:?}s before processing timelines for tokens...",
            poll_interval().as_secs()
        );
        tokio::time::sleep(poll_interval()).await;
    }
}

pub async fn perform_queue(pool: Pool<Postgres>) {
    let pool = pool.clone();
    _ = tokio::spawn(async move {
        queue_statuses_for_timelines(pool).await;
    })
    .await;
}

pub async fn perform_fetch(pool: Pool<Postgres>) {
    let pool = pool.clone();
    _ = tokio::spawn(async move {
        fetch_contexts_for_tokens_loop(&pool).await;
    })
    .await;
}
