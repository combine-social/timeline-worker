mod repository;
use repository::tokens;

#[async_std::main]
async fn main() {
    let pool = repository::establish_connection()
        .await
        .expect("Error connecting to db");
    let registrations = tokens::find_all(&pool).await;
    for reg in registrations {
        println!("Got: {:?}", reg);
    }
}
