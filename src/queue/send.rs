use amqprs::{channel::BasicPublishArguments, BasicProperties};
use serde::Serialize;

use super::{
    connect::{self},
    declare,
};

pub fn into_content<T>(message: &T) -> Result<Vec<u8>, String>
where
    T: Serialize + Sized,
{
    Ok(serde_json::to_string(message)
        .map_err(|e| -> String { e.to_string() })?
        .into_bytes())
}

pub async fn send<T>(queue: &str, message: &T) -> Result<(), String>
where
    T: Serialize + Sized,
{
    let result = connect::connect().await;
    if let Ok(connection) = result {
        declare::queue_declare(&connection.channel, queue).await?;
        connection
            .channel
            .basic_publish(
                BasicProperties::default(),
                into_content(message)?,
                BasicPublishArguments::new("", queue),
            )
            .await
            .map_err(|e| -> String { e.to_string() })?;
        Ok(())
    } else {
        Err(result.err().unwrap().to_string())
    }
}
