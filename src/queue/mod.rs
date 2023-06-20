#![cfg_attr(test, allow(dead_code))]

#[cfg(not(test))]
pub use connect::*;
#[cfg(not(test))]
pub use next::next;
#[cfg(not(test))]
pub use send::send;

mod connect;
mod consumer;
mod next;
mod send;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub use mock::*;
#[cfg(test)]
mod tests;
