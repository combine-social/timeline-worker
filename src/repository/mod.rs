#![cfg_attr(test, allow(dead_code))]

#[cfg(not(test))]
pub use connect::{connect, create_pool, Connection, ConnectionPool};

mod connect;

pub mod registrations;
pub mod tokens;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub use mock::{connect, create_pool, Connection, ConnectionPool};
