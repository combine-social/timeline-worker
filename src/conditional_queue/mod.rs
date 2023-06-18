use redis::aio::Connection;
use serde::Serialize;

use crate::{
    cache::{has, set},
    queue::{models::QueuedStatus, send, Connection as QueueConnection},
};

pub async fn send_if_not_cached<T>(
    cache_connection: &mut Connection,
    queue_connection: &QueueConnection,
    queue_name: &String,
    key: &String,
    instance: &String,
    url: &String,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    if !has(cache_connection, key).await? {
        set(cache_connection, key, value, None).await?;
        send(
            queue_connection,
            queue_name,
            &QueuedStatus {
                instance_url: instance.to_string(),
                status_url: url.to_string(),
            },
        )
        .await?;
    }
    Ok(())
}
