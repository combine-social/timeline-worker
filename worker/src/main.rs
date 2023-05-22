mod repository;
use repository::registrations;

#[async_std::main]
async fn main() {
    let connection = &mut repository::establish_connection().await;
    let registrations = registrations::find_all(connection);
    for reg in registrations {
        println!("Got: {:?}", reg);
    }
}
