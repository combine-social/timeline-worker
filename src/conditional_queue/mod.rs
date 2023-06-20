use serde::Serialize;

use crate::{
    cache::{self, Cache},
    queue::{self, Connection, QueuedStatus},
};

#[cfg(test)]
mod tests;

pub async fn send_if_not_cached<T>(
    cache: &mut Cache,
    queue_connection: &Connection,
    queue_name: &String,
    key: &String,
    instance: &String,
    url: &String,
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
            &QueuedStatus {
                instance_url: instance.to_string(),
                status_url: url.to_string(),
            },
        )
        .await?;
    } else {
        println!("Skipping {}", key);
    }
    Ok(())
}
