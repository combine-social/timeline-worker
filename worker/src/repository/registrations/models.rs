use diesel::{prelude::*};

use crate::repository::schema::registrations;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = registrations)]
pub struct Registration {
  pub id: i32,
  pub instance_url: String,
  pub registration_id: Option<String>,
  pub name: Option<String>,
  pub website: Option<String>,
  pub redirect_uri: String,
  pub client_id: String,
  pub client_secret: String,
  pub vapid_key: Option<String>,
  pub nonce: String,
}

// #[derive(Queryable)]
// pub struct Token {
//   pub id: i64,
//   pub username: String,
// 	pub access_token: String,
// 	pub token_type: Option<String>,
// 	pub scope: Option<String>,
// 	pub created_at: Option<i64>,
//   pub registration_id: i64,
//   pub fail_count: Option<i64>,
// }
