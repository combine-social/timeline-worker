use chrono::Utc;

use crate::{
    cache::{self, StatusCacheMetaData},
    conditional_queue,
    models::ContextRequest,
    repository::tokens::Token,
};

fn next_level(meta: &StatusCacheMetaData) -> StatusCacheMetaData {
    StatusCacheMetaData {
        original: meta.original.clone(),
        created_at: Utc::now(),
        index: 0,
        level: meta.level + 1,
    }
}

pub async fn send_if_needed(
    token: &Token,
    instance_url: &str,
    status_url: &String,
    status_id: &String,
    meta: &StatusCacheMetaData,
) -> Result<(), String> {
    let mut cache = cache::connect().await?;
    let own_instance = &token.registration.instance_url;
    let queue_name = format!("v2:{}", token.username);

    _ = conditional_queue::send_if_not_cached(
        &mut cache,
        &queue_name,
        &cache::status_key(own_instance, status_url),
        &ContextRequest {
            instance_url: instance_url.to_owned(),
            status_id: status_id.to_owned(),
            status_url: status_url.clone(),
        },
        &next_level(meta),
    )
    .await
    .map_err(|err| {
        error!(
            "send_if_not_cached for {} failed: {:?}",
            cache::status_key(own_instance, status_url),
            err
        );
        err
    });
    Ok(())
}
