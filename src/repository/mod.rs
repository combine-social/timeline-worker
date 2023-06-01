use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub mod registrations;
pub mod tokens;

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
