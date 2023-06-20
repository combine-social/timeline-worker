use chrono::Utc;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    conditional_queue,
    federated::{self, throttle::Throttle, ContextRequest},
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

fn next_level(meta: &StatusCacheMetaData) -> StatusCacheMetaData {
    StatusCacheMetaData {
        original: meta.original.clone(),
        created_at: Utc::now(),
        index: 0,
        level: meta.level + 1,
    }
}

pub async fn fetch_next_context(
    token: &Token,
    cache: &mut Cache,
    queue: &Connection,
    throttle: &mut Throttle,
) -> Result<(), Box<dyn std::error::Error>> {
    let queue_name = &token.username;
    if let Ok(Some(request)) = next_context_request(token, queue).await {
        let meta = metadata(&request, cache).await?;
        let key = cache::status_key(&request.instance_url, &request.status_url);
        _ = cache::set(cache, &key, &meta, None).await?;
        if meta.level <= 2 {
            if let Some(status) = federated::resolve(token, &request.status_url, throttle).await? {
                if let Some(context) = federated::get_context(&status, throttle, None).await? {
                    println!(
                        "Got {} descendants of {} from {} at index {}",
                        context.descendants.len(),
                        request.status_url,
                        meta.created_at,
                        meta.index,
                    );
                    for descentant in context.descendants {
                        let boxed = if descentant.reblogged.is_some_and(|r| r) {
                            if descentant.reblog.is_none() {
                                println!("Missing reblogged status for {}", descentant.id);
                            }
                            descentant.reblog
                        } else {
                            Some(Box::new(descentant))
                        };
                        if boxed.is_none() {
                            continue;
                        }
                        let child = boxed.unwrap();
                        if child.url.is_none() {
                            continue;
                        }
                        if let Some(child_url) = child.url {
                            _ = conditional_queue::send_if_not_cached(
                                cache,
                                queue,
                                queue_name,
                                &cache::status_key(&request.instance_url, &child_url),
                                &request.instance_url,
                                &child_url,
                                &next_level(&meta),
                            )
                            .await?;
                        }
                    }
                }
            }
        } else {
            println!(
                "Recursion too deep for child of {}, bailing.",
                meta.original
            );
        }
    } else {
        println!("Queue for {} is empty", token.username);
    }
    Ok(())
}
