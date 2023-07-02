#![cfg_attr(test, allow(dead_code))]

#[cfg(not(test))]
pub use client::*;
#[cfg(not(test))]
pub use context::*;
#[cfg(not(test))]
pub use follow::*;
#[cfg(not(test))]
pub use outbox::*;
#[cfg(not(test))]
pub use resolve::*;
#[cfg(not(test))]
pub use timeline::{get_home_timeline_page, get_notification_timeline_page};

pub use models::Page;

#[cfg(test)]
pub use mock::*;

mod client;
mod context;
mod follow;
mod models;
mod outbox;
mod resolve;
pub mod throttle;
mod timeline;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
