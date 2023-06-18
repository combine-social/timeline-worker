use std::process;

mod cache;
mod conditional_queue;
mod queue;
mod repository;

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
    let cache = cache::connect().await.unwrap_or_else(|err| {
        println!("Error connecting to Redis: {}", err);
        process::exit(-1);
    });
    println!("⚡️[server]: Cache connection up!");
    let queue = queue::connect().await.unwrap_or_else(|err| {
        println!("Error connecting to RabbitMQ: {}", err);
        process::exit(-1);
    });
    println!("⚡️[server]: Queue connection up!");
}
