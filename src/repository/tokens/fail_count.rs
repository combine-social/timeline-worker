use sqlx::{pool::PoolConnection, Postgres};

use crate::strerr::here;

use super::Token;

pub async fn update_fail_count(
    con: &mut PoolConnection<Postgres>,
    token: &Token,
    count: i32,
) -> Result<(), String> {
    sqlx::query(
        "
            update tokens
            set fail_count = $1
            where id = $2
            ",
    )
    .bind(count)
    .bind(token.id)
    .execute(con)
    .await
    .map_err(|err| here!(err))?;
    Ok(())
}
