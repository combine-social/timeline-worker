use redis::AsyncCommands;
use serde::Serialize;
use std::env;

use super::connect::Cache;

fn expire_time() -> usize {
    env::var("POLL_INTERVAL")
        .unwrap_or("300".to_owned())
        .parse::<usize>()
        .unwrap_or(60 * 5)
}

pub fn to_string<T>(meta: &T) -> Result<String, String>
where
    T: Serialize + Sized,
{
    serde_json::to_string(meta).map_err(|e| -> String { e.to_string() })
}

pub async fn set<T>(
    cache: &mut Cache,
    key: &String,
    value: &T,
    expiry: Option<usize>,
) -> Result<(), String>
where
    T: Serialize + Sized,
{
    let json = to_string(value)?;
    cache
        .connection
        .set_ex(key, json, expiry.unwrap_or(expire_time()))
        .await
        .map_err(|e| -> String { e.to_string() })
}
