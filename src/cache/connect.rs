use redis::{aio::Connection, Client};

use std::env;

use crate::strerr::here;

pub struct Cache {
    pub connection: Connection,
}

fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or("redis://localhost".to_owned())
}

fn client() -> Result<Client, String> {
    Client::open(redis_url()).map_err(|err| here!(err))
}

pub async fn connect() -> Result<Cache, String> {
    Ok(Cache {
        connection: client()?
            .get_async_connection()
            .await
            .map_err(|err| here!(err))?,
    })
}
