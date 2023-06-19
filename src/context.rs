use chrono::Utc;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    federated::{throttle::Throttle, ContextRequest},
    queue::{self, Connection},
    repository::tokens::Token,
};

async fn next_context_request(
    token: &Token,
    queue: &Connection,
) -> Result<Option<ContextRequest>, Box<dyn std::error::Error>> {
    let queue_name = &token.username;
    queue::next(queue, queue_name).await
}

async fn metadata(
    request: &ContextRequest,
    cache: &mut Cache,
) -> Result<StatusCacheMetaData, Box<dyn std::error::Error>> {
    let key = cache::status_key(&request.instance_url, &request.status_url);
    Ok(cache::get::<Option<StatusCacheMetaData>>(cache, &key)
        .await?
        .unwrap_or(StatusCacheMetaData {
            original: request.status_url.clone(),
            created_at: Utc::now(),
            index: 0,
            level: 1,
        }))
}

pub async fn fetch_next_context(
    token: &Token,
    cache: &mut Cache,
    queue: &Connection,
    throttle: &mut Throttle,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(Some(request)) = next_context_request(token, queue).await {
        let meta = metadata(&request, cache).await?;
        let key = cache::status_key(&request.instance_url, &request.status_url);
        _ = cache::set(cache, &key, &meta, None).await?;
        if meta.level <= 2 {
            todo!("get context, loop over descendants");
        }
    } else {
        println!("Queue for {} is empty", token.username);
    }
    Ok(())
}
