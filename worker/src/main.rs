mod repository;
use futures_util::StreamExt;
use repository::tokens;

#[async_std::main]
async fn main() {
    let pool = repository::establish_connection()
        .await
        .expect("Error connecting to db");
    let tokens = &mut tokens::find_all(&pool);
    while let Some(token) = tokens.next().await {
        println!("Got: {:?}", token);
    }
}
