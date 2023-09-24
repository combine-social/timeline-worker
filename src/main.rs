use std::env;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use sqlx::{Pool, Postgres};

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
mod send;
mod strerr;
mod tokens;

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

enum Mode {
    Process,
    Schedule,
}

fn mode() -> Mode {
    let mode = env::var("MODE").expect("MODE must be set to either process or schedule");
    match mode.as_str() {
        "process" => Mode::Process,
        "schedule" => Mode::Schedule,
        _ => panic!("MODE must be set to either process or schedule"),
    }
}

async fn create_pool() -> Pool<Postgres> {
    repository::create_pool()
        .await
        .expect("Failed to create connection pool")
}

#[tokio::main]
async fn main() {
    load_env();
    init_logger();
    let pool = create_pool().await;

    match mode() {
        Mode::Schedule => {
            run_loop::perform_queue(pool).await;
        }
        Mode::Process => {
            run_loop::perform_fetch(pool).await;
        }
    }
}
