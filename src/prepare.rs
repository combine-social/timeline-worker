use std::env;

use crate::{
    cache::{self},
    queue,
};

fn threshold() -> u32 {
    env::var("REPOPULATE_THRESHOLD")
        .unwrap_or("25".to_owned())
        .parse::<u32>()
        .unwrap_or(25)
}

/// Prepare to re-populate the queue.
///
/// Clears the cache so queue can be repopulated,
/// if the current queue length is less than the given threshold.
pub async fn prepare_populate_queue(queue_name: &String) -> Result<(), String> {
    let mut cache = cache::connect().await?;
    let count = queue::size(queue_name).await?;
    if count < threshold() {
        info!("Proceeding with queue {}", queue_name);
        cache::delete_keys_with_prefix(&mut cache, queue_name).await?;
    } else {
        info!("Skipping {}", queue_name);
    }
    Ok(())
}

pub async fn should_populate_queue(queue_name: &String) -> bool {
    if let Ok(count) = queue::size(queue_name).await {
        info!(
            "Size of queue {} is {}, skip threshold is {}",
            queue_name,
            count,
            threshold()
        );
        count < threshold()
    } else {
        error!("Could not get queue size");
        false
    }
}
