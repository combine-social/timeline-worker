mod cache;
mod repository;
use dotenvy::dotenv;
use futures_util::StreamExt;
use repository::tokens;

fn load_env() {
    if cfg!(debug) {
        dotenvy::from_filename(".env.local").ok();
    } else {
        dotenv().ok();
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    load_env();
    let pool = repository::establish_connection()
        .await
        .expect("Error connecting to db");
    let cache = cache::establish_connection()
        .await
        .expect("Error connecting to cache");
    let tokens = &mut tokens::find_all(&pool);
    while let Some(token) = tokens.next().await {
        println!("Got: {:?}", token);
    }
}
