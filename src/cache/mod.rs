use redis::aio::Connection;
use redis::{AsyncCommands, Client, RedisError};
use serde::{Deserialize, Serialize};
use std::env;

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

pub async fn get<T>(
    connection: &mut Connection,
    key: &String,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let json = get_string(connection, key).await?;
    serde_json::from_str(&json).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
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
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    let json = to_string(value)?;
    connection
        .set(key, json)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}
