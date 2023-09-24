use sqlx::{
    pool::PoolConnection,
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};
use std::{env, str::FromStr};

fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or("postgres://localhost/test".to_owned())
}

fn max_connections() -> u32 {
    env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or("5".to_owned())
        .parse()
        .unwrap_or(5)
}

fn options() -> PgConnectOptions {
    PgConnectOptions::from_str(database_url().as_str())
        .expect("Could not create connection options from database_url")
        .disable_statement_logging()
        .clone()
}

pub async fn create_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections())
        .connect_with(options())
        .await
}

pub async fn connect(pool: &Pool<Postgres>) -> Result<PoolConnection<Postgres>, sqlx::Error> {
    pool.acquire().await
}
