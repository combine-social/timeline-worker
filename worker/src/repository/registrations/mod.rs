use sqlx::{Error, Pool, Postgres};

use self::models::*;

pub mod models;

pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<Registration>, Error> {
    sqlx::query_as!(Registration, "select * from registrations")
        .fetch_all(pool)
        .await
}
