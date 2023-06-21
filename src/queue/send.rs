use amqprs::{
    channel::{BasicPublishArguments, Channel, QueueDeclareArguments},
    BasicProperties,
};
use serde::Serialize;

use super::connect::{self};

fn queue_args(queue: &str) -> QueueDeclareArguments {
    QueueDeclareArguments::default()
        .queue(queue.to_owned())
        .durable(true)
        .auto_delete(false)
        .finish()
}

async fn queue_declare(channel: &Channel, queue: &str) -> Result<(), String> {
    channel
        .queue_declare(queue_args(queue))
        .await
        .map_err(|e| -> String { e.to_string() })?;
    Ok(())
}

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
        queue_declare(&connection.channel, queue).await?;
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
