use sqlx::{pool::PoolConnection, Postgres};

use crate::strerr::here;

use super::Token;

pub async fn delete(con: &mut PoolConnection<Postgres>, token: &Token) -> Result<(), String> {
    sqlx::query(
        "
            delete from tokens
            where id = $1
            ",
    )
    .bind(token.id)
    .execute(con)
    .await
    .map_err(|err| here!(err))?;
    Ok(())
}
