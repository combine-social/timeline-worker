use super::{connect, consumer::StatusConsumer};
use amqprs::channel::BasicConsumeArguments;
use serde::Deserialize;
use std::sync::Arc;

pub fn into<T>(json: &str) -> Result<Option<T>, String>
where
    T: for<'a> Deserialize<'a> + Sized + Send + Sync,
{
    serde_json::from_str(json).map_err(|e| -> String { e.to_string() })
}

pub async fn next<T: for<'a> Deserialize<'a> + Sized + Send + Sync>(
    queue: &str,
) -> Result<Option<T>, String> {
    let result = connect::connect().await;
    if let Ok(connection) = result {
        let result: Arc<Option<String>> = Arc::new(None);
        let consumer: StatusConsumer = StatusConsumer::new(false, result.clone());
        let args = BasicConsumeArguments::new(queue, "");
        connection
            .channel
            .basic_consume(consumer, args)
            .await
            .map_err(|e| -> String { e.to_string() })?;
        if let Some(json) = result.as_deref() {
            into(json)
        } else {
            Ok(None)
        }
    } else {
        Err(result.err().unwrap().to_string())
    }
}
