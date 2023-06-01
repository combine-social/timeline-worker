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

#[async_std::main]
async fn main() {
    load_env();
    let pool = repository::establish_connection()
        .await
        .expect("Error connecting to db");
    let tokens = &mut tokens::find_all(&pool);
    while let Some(token) = tokens.next().await {
        println!("Got: {:?}", token);
    }
}
