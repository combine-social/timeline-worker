#[cfg(not(test))]
pub use delete::delete;
#[cfg(not(test))]
pub use fail_count::update_fail_count;
#[cfg(not(test))]
pub use find::find_by_worker_id;
#[cfg(test)]
pub use mock::{delete, find_by_worker_id, update_fail_count};

pub use models::*;

#[cfg(not(test))]
mod delete;
#[cfg(not(test))]
mod fail_count;
#[cfg(not(test))]
mod find;

mod models;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
