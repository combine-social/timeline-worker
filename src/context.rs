use chrono::Utc;
use megalodon::entities::Status;
use url::Url;

use crate::{
    cache::{self, Cache, StatusCacheMetaData},
    federated::{self, OriginId},
    models::ContextRequest,
    queue::{self},
    repository::tokens::Token,
    send,
    strerr::here,
};

async fn next_context_request(token: &Token) -> Result<Option<ContextRequest>, String> {
    let queue_name = format!("v2:{}", token.username);
    queue::next(&queue_name).await
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
                        match child.origin_id().await {
                            Ok(id) => {
                                _ = send::send_if_needed(
                                    token,
                                    &request.instance_url.clone(),
                                    &child_url,
                                    &id,
                                    &meta,
                                )
                                .await;
                            }
                            Err(err) => {
                                warn!("Could not get origin id for {}: {}", child.id, err);
                            }
                        }
                    }
                }
            } else {
                error!("get_context for {:?} failed", request);
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
