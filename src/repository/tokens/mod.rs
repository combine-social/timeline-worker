#[cfg(not(test))]
pub use find::find_all;
#[cfg(test)]
pub use mock::find_all;

pub use models::*;

#[cfg(not(test))]
mod find;

mod models;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
