use std::process;

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

#[cfg(test)]
mod tests;

fn load_env() {
    dotenvy::from_filename(".env.local").ok();
    dotenvy::from_filename_override(".env").ok();
}

#[tokio::main]
async fn main() {
    load_env();
    let db = repository::create_pool().await.unwrap_or_else(|err| {
        println!("Error connecting to Postgres: {}", err);
        process::exit(-1);
    });
    println!("⚡️[server]: DB connection up!");
    federated::throttle::initialize();
    run_loop::perform_loop(db).await;
}
