use crate::strerr::here;

use super::super::registrations::Registration;
use super::Token;
use sqlx::pool::PoolConnection;
use sqlx::types::JsonRawValue;
use sqlx::Postgres;
use sqlx::{postgres::PgRow, FromRow, Row};

pub async fn find_by_worker_id(
    con: &mut PoolConnection<Postgres>,
    worker_id: i32,
) -> Result<Vec<Token>, String> {
    let rows = sqlx::query(
        "
                select
                    t.*,
                    to_json(r.*) as registration
                from registrations r 
                join tokens t 
                    on r.id = t.registration_id
                where t.worker_id = $1
            ",
    )
    .bind(worker_id)
    .fetch_all(con)
    .await
    .map_err(|err| here!(err))?;
    let tokens: Vec<Token> = rows
        .iter()
        .map(Token::from_row)
        .map(|result| result.expect("Token::from_row error"))
        .collect();
    Ok(tokens)
}

fn raw_value<'a>(row: &'a PgRow, key: &str) -> Result<&'a JsonRawValue, sqlx::Error> {
    row.try_get(key)
}

fn result(row: &PgRow) -> Result<Registration, Box<dyn std::error::Error>> {
    let json =
        raw_value(row, "registration").map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    serde_json::from_str(json.to_string().as_str())
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

impl FromRow<'_, PgRow> for Token {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        match result(row) {
            Ok(registration) => Ok(Self {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                access_token: row.try_get("access_token")?,
                token_type: row.try_get("token_type")?,
                scope: row.try_get("scope")?,
                created_at: row.try_get("created_at")?,
                fail_count: row.try_get("fail_count")?,
                registration,
                worker_id: row.try_get("worker_id")?,
                ping_at: row.try_get("ping_at")?,
            }),
            // ColumnNotFound was the least bad error I could find to map to
            Err(error) => Err(sqlx::Error::ColumnNotFound(error.to_string())),
        }
    }
}
