use redis::AsyncCommands;

use super::{connect::Cache, get};

pub async fn delete_keys_with_prefix(cache: &mut Cache, prefix: &String) -> Result<(), String> {
    for key in get::get_keys_with_prefix(cache, prefix).await? {
        cache
            .connection
            .del::<String, ()>(key)
            .await
            .map_err(|e| -> String { e.to_string() })?;
    }
    Ok(())
}

pub async fn delete_key(cache: &mut Cache, key: &String) -> Result<(), String> {
    cache
        .connection
        .del::<String, ()>(key.to_owned())
        .await
        .map_err(|e| -> String { e.to_string() })
}
