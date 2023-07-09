use super::super::connect::Connection;
use super::Token;

pub async fn update_fail_count(
    con: &mut Connection,
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
    .execute(&mut con.connection)
    .await
    .map_err(|err| err.to_string())?;
    Ok(())
}
