use redis::{aio::Connection, AsyncCommands};
use serde::Serialize;
use std::env;

fn expire_time() -> usize {
    env::var("POLL_INTERVAL")
        .unwrap_or("300".to_owned())
        .parse::<usize>()
        .unwrap_or(60 * 5)
}

fn to_string<T>(meta: &T) -> Result<String, Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    serde_json::to_string(meta).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub async fn set<T>(
    connection: &mut Connection,
    key: &String,
    value: &T,
    expiry: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    let json = to_string(value)?;
    connection
        .set_ex(key, json, expiry.unwrap_or(expire_time()))
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}
