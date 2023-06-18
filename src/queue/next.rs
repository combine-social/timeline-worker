use super::{connect::Connection, consumer::StatusConsumer};
use amqprs::channel::BasicConsumeArguments;
use serde::Deserialize;
use std::sync::Arc;

pub fn into<T>(json: &str) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a> + Sized + Send + Sync,
{
    serde_json::from_str(json).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub async fn next<T: for<'a> Deserialize<'a> + Sized + Send + Sync>(
    connection: &Connection,
    queue: &str,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let result: Arc<Option<String>> = Arc::new(None);
    let consumer: StatusConsumer = StatusConsumer::new(false, result.clone());
    let args = BasicConsumeArguments::new(queue, "");
    connection
        .channel
        .basic_consume(consumer, args)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    if let Some(json) = result.as_deref() {
        into(json)
    } else {
        Ok(None)
    }
}
