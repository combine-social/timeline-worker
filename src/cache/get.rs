use redis::AsyncCommands;
use serde::Deserialize;

use super::connect::Cache;

pub async fn has(cache: &mut Cache, key: &String) -> Result<bool, String> {
    let result: Result<u32, redis::RedisError> = cache.connection.exists(key).await;
    if let Ok(exists) = result {
        Ok(exists != 0)
    } else {
        Err(result.err().unwrap().to_string())
    }
}

async fn get_string(cache: &mut Cache, key: &String) -> Result<String, String> {
    cache
        .connection
        .get(key)
        .await
        .map_err(|e| -> String { e.to_string() })
}

pub fn deserialize<T>(value: &str) -> Result<T, String>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    serde_json::from_str(value).map_err(|e| -> String { e.to_string() })
}

pub async fn get<T>(cache: &mut Cache, key: &String) -> Result<T, String>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    deserialize(&get_string(cache, key).await?)
}

pub async fn get_keys_with_prefix(
    cache: &mut Cache,
    prefix: &String,
) -> Result<Vec<String>, String> {
    cache
        .connection
        .keys::<String, Vec<String>>(format!("{prefix}*"))
        .await
        .map_err(|e| -> String { e.to_string() })
}
