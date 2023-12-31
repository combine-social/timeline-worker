#![cfg_attr(test, allow(dead_code))]

#[cfg(not(test))]
pub use client::*;
#[cfg(not(test))]
pub use context::*;
#[cfg(not(test))]
pub use detect::*;
#[cfg(not(test))]
pub use follow::*;
#[cfg(not(test))]
pub use outbox::*;
pub use resolve::{resolve, set_do_not_resolve};
#[cfg(not(test))]
pub use timeline::{get_home_timeline_page, get_notification_timeline_page};

pub use models::Page;
pub use origin_id::OriginId;

#[cfg(test)]
pub use mock::*;

mod client;
mod context;
mod detect;
mod follow;
mod models;
mod origin_id;
mod outbox;
mod resolve;
mod throttle;
mod timeline;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
