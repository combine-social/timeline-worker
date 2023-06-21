use super::{connect, declare};

pub async fn size(queue_name: &str) -> Result<u32, String> {
    let connection = connect::connect().await.map_err(|err| err.to_string())?;
    declare::queue_declare(&connection.channel, queue_name).await
}
