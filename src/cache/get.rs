use redis::AsyncCommands;
use serde::Deserialize;

use super::connect::Cache;

pub async fn has(cache: &mut Cache, key: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let exists: u32 = cache.connection.exists(key).await?;
    Ok(exists != 0)
}

async fn get_string(cache: &mut Cache, key: &String) -> Result<String, Box<dyn std::error::Error>> {
    cache
        .connection
        .get(key)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub fn deserialize<T>(value: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    serde_json::from_str(value).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub async fn get<T>(cache: &mut Cache, key: &String) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    deserialize(&get_string(cache, key).await?)
}
