use chrono::Utc;
use megalodon::entities::Status;
use url::Url;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    conditional_queue,
    federated::{self, throttle::Throttle},
    models::ContextRequest,
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

fn status_or_reblog(status: Status) -> Status {
    if status.reblog.is_some() {
        *status.reblog.unwrap()
    } else {
        status
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
        let key = cache::status_key(&request.instance_url, &request.status_id);
        cache::set(cache, &key, &meta, None).await?;
        if meta.level <= 2 {
            _ = federated::resolve(token, &request.status_id, throttle).await?;
            if let Some(host) = Url::parse(&request.status_url)?.host_str() {
                if let Some(context) = federated::get_context(
                    &host.to_string(), // [comment to force cargo fmt to break line]
                    &request.status_id,
                    throttle,
                    None, // todo: use cached host sns detection
                )
                .await?
                {
                    println!(
                        "Got {} descendants of {} from {} at index {}",
                        context.descendants.len(),
                        request.status_url,
                        meta.created_at,
                        meta.index,
                    );
                    for descentant in context.descendants {
                        let child = status_or_reblog(descentant);
                        if child.url.is_none() {
                            println!("Missing url for {}", child.id);
                            continue;
                        }
                        if let Some(child_url) = child.url {
                            conditional_queue::send_if_not_cached(
                                cache,
                                queue,
                                queue_name,
                                &cache::status_key(&request.instance_url, &child_url),
                                &ContextRequest {
                                    instance_url: request.instance_url.clone(),
                                    status_id: child.id,
                                    status_url: child_url,
                                },
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
