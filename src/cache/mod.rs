#![cfg_attr(test, allow(dead_code))]

#[cfg(not(test))]
pub use connect::*;
#[cfg(not(test))]
pub use delete::*;
#[cfg(not(test))]
pub use get::{get, get_keys_with_prefix, has};
#[cfg(not(test))]
pub use set::set;

mod connect;
mod delete;
mod get;
mod key;
mod set;

pub use key::*;

mod models;
pub use models::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub use mock::*;
#[cfg(test)]
mod tests;
