#![cfg_attr(test, allow(dead_code))]

#[cfg(not(test))]
pub use connect::*;
#[cfg(not(test))]
pub use declare::*;
#[cfg(not(test))]
pub use next::next;
#[cfg(not(test))]
pub use send::send;
#[cfg(not(test))]
pub use size::size;

mod connect;
mod consumer;
mod declare;
mod next;
mod send;
mod size;

#[cfg(test)]
mod mock;
#[cfg(test)]
pub use mock::*;
#[cfg(test)]
mod tests;
