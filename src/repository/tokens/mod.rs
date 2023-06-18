use futures::stream::StreamExt; // enable map on stream of futures
use futures::Stream;
use sqlx::pool::PoolConnection;
use sqlx::types::JsonRawValue;
use sqlx::{postgres::PgRow, FromRow, Postgres, Row};

use self::models::*;
use super::registrations::models::*;

pub mod models;

pub fn find_all(con: &mut PoolConnection<Postgres>) -> impl Stream<Item = Token> + '_ {
    sqlx::query(
        "
                select
                    t.*,
                    to_json(r.*) as registration
                from registrations r 
                join tokens t 
                    on r.id = t.registration_id
            ",
    )
    .fetch(con)
    .map(|row| Token::from_row(&row.expect("PgRow unwrap error")))
    .map(|result| result.expect("Token::from_row error"))
}

fn raw_value<'a>(row: &'a PgRow, key: &str) -> Result<&'a JsonRawValue, sqlx::Error> {
    row.try_get(key)
}

fn result(row: &PgRow) -> Result<Registration, Box<dyn std::error::Error>> {
    let json =
        raw_value(&row, "registration").map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    serde_json::from_str(json.to_string().as_str())
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

impl FromRow<'_, PgRow> for Token {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        match result(&row) {
            Ok(registration) => Ok(Self {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                access_token: row.try_get("access_token")?,
                token_type: row.try_get("token_type")?,
                scope: row.try_get("scope")?,
                created_at: row.try_get("created_at")?,
                fail_count: row.try_get("fail_count")?,
                registration,
            }),
            // ColumnNotFound was the least bad error I could find to map to
            Err(error) => Err(sqlx::Error::ColumnNotFound(error.to_string())),
        }
    }
}
