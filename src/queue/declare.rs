use amqprs::channel::{Channel, QueueDeclareArguments};

use crate::strerr::here;

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
        .map(|result| {
            result
                .unwrap_or_else(|| {
                    warn!("queue_declare None result for {:?}", queue);
                    (queue.to_string(), 0, 0)
                })
                .1
        })
        .map_err(|e| -> String { here!(e) })
}
