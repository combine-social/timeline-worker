use futures::stream::StreamExt; // enable map on stream of futures
use futures::Stream;
use sqlx::types::JsonRawValue;
use sqlx::{postgres::PgRow, FromRow, Pool, Postgres, Row};

use self::models::*;

pub mod models;

pub fn find_all(pool: &Pool<Postgres>) -> impl Stream<Item = Token> {
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
    .fetch(pool)
    .map(|row| Token::from_row(&row.expect("PgRow unwrap error")))
    .map(|result| result.expect("Token::from_row error"))
}

impl FromRow<'_, PgRow> for Token {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let json: &JsonRawValue = row.try_get("registration")?;
        // FIXME: skip the to_string middleman
        let registration = serde_json::from_str(json.to_string().as_str()).unwrap();
        Ok(Self {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            access_token: row.try_get("access_token")?,
            token_type: row.try_get("token_type")?,
            scope: row.try_get("scope")?,
            created_at: row.try_get("created_at")?,
            fail_count: row.try_get("fail_count")?,
            registration,
        })
    }
}
