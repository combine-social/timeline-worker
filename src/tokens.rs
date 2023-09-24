use chrono::Utc;
use sqlx::{Pool, Postgres};

use crate::{
    cache,
    repository::{self, tokens::Token},
    strerr::here,
};

const EXPIRY: usize = 60 * 60 * 24;

pub async fn refresh_tokens(pool: &Pool<Postgres>, worker_id: i32) -> Result<(), String> {
    let mut cache = cache::connect().await?;
    let mut connection = repository::connect(pool).await.map_err(|err| here!(err))?;
    let persistent = repository::tokens::find_by_worker_id(&mut connection, worker_id).await?;
    for token in persistent.iter() {
        let key = cache::token_key(worker_id, token.id);
        let mut pinged = token.to_owned();
        let cached: Result<Token, String> = cache::get(&mut cache, &key).await;
        pinged.ping_at = cached.ok().and_then(|t| t.ping_at);
        let key = cache::token_key(worker_id, token.id);
        cache::set(&mut cache, &key, &pinged, Some(EXPIRY)).await?;
    }
    Ok(())
}

pub async fn get_tokens(worker_id: i32) -> Result<Vec<Token>, String> {
    let mut cache = cache::connect().await?;
    let keys = cache::get_keys_with_prefix(&mut cache, &cache::tokens_prefix(worker_id)).await?;
    let mut tokens = vec![];
    for key in keys {
        let token = cache::get(&mut cache, &key).await?;
        tokens.push(token);
    }
    Ok(tokens)
}

pub async fn update_token_fail_count(
    pool: &Pool<Postgres>,
    worker_id: i32,
    token: &Token,
    count: i32,
) -> Result<(), String> {
    if token.fail_count == Some(count) {
        return Ok(());
    }
    let mut cache = cache::connect().await?;
    let key = format!("{}:{}", cache::tokens_prefix(worker_id), token.id);
    cache::set(&mut cache, &key, token, Some(EXPIRY)).await?;
    let mut connection = repository::connect(pool).await.map_err(|err| here!(err))?;
    repository::tokens::update_fail_count(&mut connection, token, count).await?;
    Ok(())
}

pub async fn delete_token(
    pool: &Pool<Postgres>,
    worker_id: i32,
    token: &Token,
) -> Result<(), String> {
    let mut cache = cache::connect().await?;
    let key = cache::token_key(worker_id, token.id);
    cache::delete_key(&mut cache, &key).await?;
    let mut connection = repository::connect(pool).await.map_err(|err| here!(err))?;
    repository::tokens::delete(&mut connection, token).await
}

pub async fn ping_token(worker_id: i32, token: &Token) -> Result<Token, String> {
    let mut token = token.to_owned();
    token.ping_at = Some(Utc::now().timestamp() as i32);
    let mut cache = cache::connect().await?;
    let key = cache::token_key(worker_id, token.id);
    cache::set(&mut cache, &key, &token, Some(EXPIRY)).await?;
    Ok(token)
}

pub async fn clear_token_ping(worker_id: i32, token: &Token) -> Result<Token, String> {
    let mut token = token.to_owned();
    token.ping_at = None;
    let mut cache = cache::connect().await?;
    let key = cache::token_key(worker_id, token.id);
    cache::set(&mut cache, &key, &token, Some(EXPIRY)).await?;
    Ok(token)
}
