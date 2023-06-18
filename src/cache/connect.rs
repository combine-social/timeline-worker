use redis::{aio::Connection, Client, RedisError};

use std::env;

pub struct Cache {
    pub connection: Connection,
}

fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or("redis://localhost".to_owned())
}

fn client() -> Result<Client, RedisError> {
    Client::open(redis_url())
}

pub async fn connect() -> Result<Cache, RedisError> {
    Ok(Cache {
        connection: client()?.get_async_connection().await?,
    })
}
