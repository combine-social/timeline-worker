use crate::strerr::here;

use super::{connect, declare};

pub async fn size(queue_name: &str) -> Result<u32, String> {
    let connection = connect::connect().await.map_err(|err| here!(err))?;
    declare::queue_declare(&connection.channel, queue_name).await
}
