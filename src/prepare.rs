use std::env;

use crate::{
    cache::{self, Cache},
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
pub async fn prepare_populate_queue(cache: &mut Cache, queue_name: &String) -> Result<(), String> {
    let count = queue::size(queue_name).await?;
    println!(
        "Size of queue {} is {}, skip threshold is {}",
        queue_name,
        count,
        threshold()
    );
    if count < threshold() {
        println!("Proceeding with queue {}", queue_name);
        cache::delete_keys_with_prefix(cache, queue_name).await?;
    } else {
        println!("Skipping {}", queue_name);
    }
    Ok(())
}
