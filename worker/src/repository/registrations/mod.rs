use self::models::*;
use diesel::prelude::*;

pub mod models;

pub fn find_all(connection: &mut PgConnection) -> Vec<Registration> {
  use super::schema::registrations::dsl::*;

  registrations
    .load::<Registration>(connection)
    .expect("Error loading registrations")
}
