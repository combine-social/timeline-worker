use chrono::Utc;
use megalodon::entities::Status;
use url::Url;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    conditional_queue,
    federated::{self, OriginId},
    models::ContextRequest,
    queue::{self},
    repository::tokens::Token,
    strerr::here,
};

async fn next_context_request(token: &Token) -> Result<Option<ContextRequest>, String> {
    let queue_name = &token.username;
    queue::next(queue_name).await
}

async fn metadata(
    own_instance: &String,
    request: &ContextRequest,
    cache: &mut Cache,
) -> Result<StatusCacheMetaData, String> {
    let key = cache::status_key(own_instance, &request.status_url);
    if !cache::has(cache, &key).await? {
        return Ok(StatusCacheMetaData {
            original: request.status_url.clone(),
            created_at: Utc::now(),
            index: 0,
            level: 1,
        });
    }
    let result = cache::get::<Option<StatusCacheMetaData>>(cache, &key).await;
    if let Ok(value) = result {
        Ok(value.unwrap_or(StatusCacheMetaData {
            original: request.status_url.clone(),
            created_at: Utc::now(),
            index: 0,
            level: 1,
        }))
    } else {
        let msg = result.err().unwrap();
        error!("Error getting metadata for {:?}: {:?}", request, msg);
        Err(here!(msg))
    }
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

fn request_host(request: &ContextRequest) -> Result<String, String> {
    let result = Url::parse(&request.status_url);
    if let Ok(url) = result {
        if let Some(host) = url.host_str() {
            Ok(host.to_string())
        } else {
            let message = format!("Missing host in {}", url);
            error!("{}", message);
            Err(message)
        }
    } else {
        Err(result.err().unwrap().to_string())
    }
}

pub async fn fetch_next_context(token: &Token) -> Result<bool, String> {
    let mut cache = cache::connect().await?;
    let own_instance = &token.registration.instance_url;
    let queue_name = &token.username;
    let result = next_context_request(token).await;
    info!("next_context_request result: {:?}", &result);
    if let Ok(Some(request)) = result {
        let meta = metadata(own_instance, &request, &mut cache).await?;
        let key = cache::status_key(own_instance, &request.status_url);
        cache::set(&mut cache, &key, &meta, None).await?;
        if meta.level <= 2 {
            federated::resolve(token, &request.status_url).await;
            if let Some(context) =
                federated::get_context(&request_host(&request)?, &request.status_id)
                    .await
                    .map_err(|err| {
                        error!(
                            "fetch_next_context error in get_context for {}#{}: {:?}",
                            request_host(&request).unwrap_or("unknown-host".to_string()),
                            &request.status_id,
                            err
                        );
                        err
                    })?
            {
                info!(
                    "Got {} descendants of {} from {} at index {}",
                    context.descendants.len(),
                    request.status_url,
                    meta.created_at,
                    meta.index,
                );
                for descentant in context.descendants {
                    let child = status_or_reblog(descentant);
                    if child.url.is_none() {
                        warn!("Missing url for {}", child.id);
                        continue;
                    }
                    if let Some(child_url) = child.url.clone() {
                        conditional_queue::send_if_not_cached(
                            &mut cache,
                            queue_name,
                            &cache::status_key(own_instance, &child_url),
                            &ContextRequest {
                                instance_url: request.instance_url.clone(),
                                status_id: child.origin_id()?,
                                status_url: child_url.clone(),
                            },
                            &next_level(&meta),
                        )
                        .await
                        .map_err(|err| {
                            error!(
                                "send_if_not_cached for {} failed: {:?}",
                                cache::status_key(own_instance, &child_url),
                                err
                            );
                            err
                        })?;
                    }
                }
            } else {
                error!("get_contect for {:?} failed", request);
            }
        } else {
            warn!(
                "Recursion too deep for child of {}, bailing.",
                meta.original
            );
        }
        Ok(true)
    } else {
        info!("Queue for {} is empty", token.username);
        Ok(false)
    }
}
