use std::{env, sync::Arc, time::Duration};

use futures_util::StreamExt;

use crate::{
    context, federated, home, notification, prepare,
    repository::{
        self,
        tokens::{self, Token},
        Connection, ConnectionPool,
    },
    strerr::here,
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

async fn connect(db: Arc<ConnectionPool>) -> Result<Connection, String> {
    info!("Connecting to db...");
    repository::connect(&db).await.map_err(|err| {
        let err = here!(err);
        error!("Error connecting to db: {}", err);
        err
    })
}

/// Verify token
///
/// Check that a token can be used to create an authenticated client
/// and update the fail count. Delete tokens whose fail count goes
/// over the set threshold.
async fn verify_token(token: &Token, db: Arc<ConnectionPool>) -> Result<(), String> {
    match connect(db).await {
        Ok(mut connection) => {
            let result = federated::has_verified_authenticated_client(token).await;
            if result.is_ok() {
                repository::tokens::update_fail_count(&mut connection, token, 0).await?;
            } else {
                let fail_count = token.fail_count.unwrap_or(0) + 1;
                if fail_count > max_fail_count() {
                    warn!(
                        "Fail-count threshold exceeded for {}, deleting",
                        &token.username
                    );
                    repository::tokens::delete(&mut connection, token).await?;
                } else {
                    repository::tokens::update_fail_count(&mut connection, token, fail_count)
                        .await?;
                }
            }
            result
        }
        Err(err) => {
            error!("Error connecting to db: {:?}", err);
            Err(err)
        }
    }
}

async fn fetch_contexts_for_tokens_loop(db: Arc<ConnectionPool>) {
    loop {
        if let Ok(mut connection) = connect(db.clone()).await {
            tokens::find_by_worker_id(&mut connection, worker_id())
                .for_each_concurrent(None, |token| {
                    let db = db.clone();
                    async move {
                        if verify_token(&token, db).await.is_ok() {
                            info!(
                                "fetch_contexts_for_tokens_loop got token for: {:?}",
                                &token.username
                            );
                            tokio::spawn(async move {
                                while context::fetch_next_context(&token.to_owned())
                                    .await
                                    .is_ok_and(|more| more)
                                {
                                    info!("Fetching next context for: {}", &token.username);
                                }
                            });
                        } else {
                            warn!("Could not verify token for {}", &token.username);
                        }
                    }
                })
                .await;
        }
        info!(
            "Waiting: {:?}s before fetching contexts for tokens...",
            process_interval().as_secs()
        );
        tokio::time::sleep(process_interval()).await;
    }
}

async fn perform_repopulate(token: Token, db: Arc<ConnectionPool>) {
    info!(
        "queue_statuses_for_timelines got token for: {:?}",
        &token.username
    );
    if verify_token(&token, db).await.is_ok() {
        let queue_name = format!("v2:{}", &token.username);
        if prepare::should_populate_queue(&queue_name).await {
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
        warn!("Could not verify token for: {:?}", &token.username);
    }
}

async fn queue_statuses_for_timelines(db: Arc<ConnectionPool>) {
    loop {
        if let Ok(mut connection) = connect(db.clone()).await {
            tokens::find_by_worker_id(&mut connection, worker_id())
                .for_each_concurrent(None, |token| {
                    let db = db.clone();
                    async move {
                        tokio::spawn(async {
                            perform_repopulate(token, db).await;
                        });
                    }
                })
                .await;
        }
        info!(
            "Waiting: {:?}s before processing timelines for tokens...",
            poll_interval().as_secs()
        );
        tokio::time::sleep(poll_interval()).await;
    }
}

pub async fn perform_queue(db: ConnectionPool) {
    let db = Arc::new(db);
    _ = tokio::spawn(async move {
        queue_statuses_for_timelines(db).await;
    })
    .await;
}

pub async fn perform_fetch(db: ConnectionPool) {
    let db = Arc::new(db);
    _ = tokio::spawn(async move {
        fetch_contexts_for_tokens_loop(db).await;
    })
    .await;
}
