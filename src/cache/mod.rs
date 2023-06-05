use redis::aio::Connection;
use redis::{AsyncCommands, Client, RedisError};
use std::env;

use self::models::StatusCacheMetaData;

pub mod models;

fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or("redis://localhost".to_owned())
}

fn client() -> Result<Client, RedisError> {
    Client::open(redis_url())
}

pub async fn connect() -> Result<Connection, RedisError> {
    client()?.get_async_connection().await
}

async fn get_string(
    connection: &mut Connection,
    key: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    connection
        .get(key)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub async fn get_meta(
    connection: &mut Connection,
    key: &String,
) -> Result<StatusCacheMetaData, Box<dyn std::error::Error>> {
    let json = get_string(connection, key).await?;
    serde_json::from_str(&json).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

fn meta_to_string(meta: &StatusCacheMetaData) -> Result<String, Box<dyn std::error::Error>> {
    serde_json::to_string(meta).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub async fn set_meta(
    connection: &mut Connection,
    key: &String,
    value: &StatusCacheMetaData,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = meta_to_string(value)?;
    connection
        .set(key, json)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}
