use crate::strerr::here;

use super::super::connect::Connection;
use super::Token;

pub async fn delete(con: &mut Connection, token: &Token) -> Result<(), String> {
    sqlx::query(
        "
            delete from tokens
            where id = $1
            ",
    )
    .bind(token.id)
    .execute(&mut con.connection)
    .await
    .map_err(|err| here!(err))?;
    Ok(())
}
