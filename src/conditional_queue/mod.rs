use serde::Serialize;

use crate::{
    cache::{self, Cache},
    models::ContextRequest,
    queue::{self, Connection},
};

#[cfg(test)]
mod tests;

pub async fn send_if_not_cached<T>(
    cache: &mut Cache,
    queue_connection: &Connection,
    queue_name: &String,
    key: &String,
    instance: &String,
    status_url: &String,
    status_id: &String,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    if !cache::has(cache, key).await? {
        println!("Queueing {}", key);
        cache::set(cache, key, value, None).await?;
        queue::send(
            queue_connection,
            queue_name,
            &ContextRequest {
                instance_url: instance.to_string(),
                status_id: status_id.to_string(),
                status_url: status_url.to_string(),
            },
        )
        .await?;
    } else {
        println!("Skipping {}", key);
    }
    Ok(())
}
