use sqlx::{pool::PoolConnection, postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub struct ConnectionPool {
    pub pool: Pool<Postgres>,
}

pub struct Connection {
    pub connection: PoolConnection<Postgres>,
}

fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or("postgres://localhost/test".to_owned())
}

fn max_connections() -> u32 {
    env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or("5".to_owned())
        .parse()
        .unwrap_or(5)
}

pub async fn create_pool() -> Result<ConnectionPool, sqlx::Error> {
    Ok(ConnectionPool {
        pool: PgPoolOptions::new()
            .max_connections(max_connections())
            .connect(&database_url())
            .await?,
    })
}

pub async fn connect(pool: &ConnectionPool) -> Result<Connection, sqlx::Error> {
    Ok(Connection {
        connection: pool.pool.acquire().await?,
    })
}
