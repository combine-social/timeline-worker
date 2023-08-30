use crate::{
    cache,
    repository::{self, tokens::Token},
    strerr::here,
};

const EXPIRY: usize = 60 * 60 * 24;

pub async fn get_tokens(worker_id: i32) -> Result<Vec<Token>, String> {
    let mut cache = cache::connect().await?;
    let tokens = cache::get_keys_with_prefix(&mut cache, &cache::tokens_prefix(worker_id))
        .await?
        .iter()
        .map(|token| serde_json::from_str(token))
        .filter_map(|row| row.ok())
        .collect::<Vec<Token>>();
    if !tokens.is_empty() {
        return Ok(tokens);
    }
    let pool = repository::create_pool().await.map_err(|err| here!(err))?;
    let mut connection = repository::connect(&pool).await.map_err(|err| here!(err))?;
    let tokens = repository::tokens::find_by_worker_id(&mut connection, worker_id).await?;
    for token in tokens.iter() {
        let key = format!("{}:{}", cache::tokens_prefix(worker_id), token.id);
        cache::set(&mut cache, &key, token, Some(EXPIRY)).await?;
    }
    Ok(tokens)
}

pub async fn update_token_fail_count(
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
    let pool = repository::create_pool().await.map_err(|err| here!(err))?;
    let mut connection = repository::connect(&pool).await.map_err(|err| here!(err))?;
    repository::tokens::update_fail_count(&mut connection, token, count).await?;
    Ok(())
}

pub async fn delete_token(worker_id: i32, token: &Token) -> Result<(), String> {
    let mut cache = cache::connect().await?;
    let key = format!("{}:{}", cache::tokens_prefix(worker_id), token.id);
    cache::delete_key(&mut cache, &key).await?;
    let pool = repository::create_pool().await.map_err(|err| here!(err))?;
    let mut connection = repository::connect(&pool).await.map_err(|err| here!(err))?;
    repository::tokens::delete(&mut connection, token).await
}
