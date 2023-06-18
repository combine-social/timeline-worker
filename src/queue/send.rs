use amqprs::{
    channel::{BasicPublishArguments, Channel, QueueDeclareArguments},
    BasicProperties,
};
use serde::Serialize;

use super::Connection;

fn queue_args(queue: &str) -> QueueDeclareArguments {
    QueueDeclareArguments::default()
        .queue(queue.to_owned())
        .durable(true)
        .auto_delete(false)
        .finish()
}

async fn queue_declare(channel: &Channel, queue: &str) -> Result<(), Box<dyn std::error::Error>> {
    channel
        .queue_declare(queue_args(queue))
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    Ok(())
}

fn into_content<T>(message: &T) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    Ok(serde_json::to_string(message)
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?
        .into_bytes())
}

pub async fn send<T>(
    connection: &Connection,
    queue: &str,
    message: &T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    queue_declare(&connection.channel, queue).await?;
    connection
        .channel
        .basic_publish(
            BasicProperties::default(),
            into_content(message)?,
            BasicPublishArguments::new("", queue),
        )
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    Ok(())
}
