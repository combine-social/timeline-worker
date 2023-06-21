use amqprs::channel::{Channel, QueueDeclareArguments};

fn queue_args(queue: &str) -> QueueDeclareArguments {
    QueueDeclareArguments::default()
        .queue(queue.to_owned())
        .durable(true)
        .auto_delete(false)
        .finish()
}

/// Declare a queue and return the message count
pub async fn queue_declare(channel: &Channel, queue: &str) -> Result<u32, String> {
    channel
        .queue_declare(queue_args(queue))
        .await
        .map(|result| result.unwrap_or((queue.to_string(), 0, 0)).1)
        .map_err(|e| -> String { e.to_string() })
}
