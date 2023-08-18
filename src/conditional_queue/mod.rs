use std::fmt::Debug;

use serde::Serialize;

use crate::{
    cache::{self, Cache},
    models::ContextRequest,
    queue::{self},
};

#[cfg(test)]
mod tests;

pub async fn send_if_not_cached<T>(
    cache: &mut Cache,
    queue_name: &str,
    key: &String,
    request: &ContextRequest,
    value: &T,
) -> Result<(), String>
where
    T: Serialize + Sized + Debug,
{
    if !cache::has(cache, key).await? {
        info!(
            "Adding to queue {:?}: {:?} with cache key {:?}: {:?}",
            queue_name, request, key, value,
        );
        cache::set(cache, key, value, None).await?;
        queue::send(queue_name, request).await?;
    } else {
        info!("Skipping {}", key);
    }
    Ok(())
}
