pub use get::{get, has};
use redis::aio::Connection;
use redis::{Client, RedisError};
pub use set::set;
use std::env;

mod get;
pub mod models;
mod set;

fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or("redis://localhost".to_owned())
}

fn client() -> Result<Client, RedisError> {
    Client::open(redis_url())
}

pub async fn connect() -> Result<Connection, RedisError> {
    client()?.get_async_connection().await
}
