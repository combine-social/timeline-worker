use std::process;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};

mod cache;
mod conditional_queue;
mod context;
mod federated;
mod home;
mod models;
mod notification;
mod prepare;
mod queue;
mod queue_statuses;
mod repository;
mod run_loop;
mod strerr;

#[macro_use]
extern crate log;
extern crate openssl_probe;
extern crate simplelog;

#[cfg(test)]
mod tests;

fn load_env() {
    dotenvy::from_filename(".env.local").ok();
    dotenvy::from_filename_override(".env").ok();
    openssl_probe::init_ssl_cert_env_vars();
}

fn init_logger() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}

#[tokio::main]
async fn main() {
    load_env();
    init_logger();
    let db = repository::create_pool().await.unwrap_or_else(|err| {
        error!("Error connecting to Postgres: {}", err);
        process::exit(-1);
    });
    info!("⚡️[server]: DB connection up!");
    run_loop::perform_loop(db).await;
}
