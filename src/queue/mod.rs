use std::{env, sync::Arc};

use amqprs::{
    channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments},
    connection::{self, OpenConnectionArguments},
    BasicProperties,
};
use serde::{Deserialize, Serialize};

use self::consumer::StatusConsumer;

pub mod models;

mod consumer;

pub struct Connection {
    _con: connection::Connection, // Hang on to connection to keep channel open
    chan: Channel,
}

fn url() -> String {
    env::var("QUEUE_URL").unwrap_or("amqp://localhost".to_owned())
}

pub async fn connect() -> Result<Connection, amqprs::error::Error> {
    let args = OpenConnectionArguments::try_from(url().as_str())?;
    let con = connection::Connection::open(&args).await?;
    let chan = con.open_channel(None).await?;
    Ok(Connection { _con: con, chan })
}

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
    con: &Connection,
    queue: &str,
    message: &T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    queue_declare(&con.chan, queue).await?;
    con.chan
        .basic_publish(
            BasicProperties::default(),
            into_content(message)?,
            BasicPublishArguments::new("", queue),
        )
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    Ok(())
}

pub async fn next<T: for<'a> Deserialize<'a> + Sized + Send + Sync>(
    connection: &Connection,
    queue: &str,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let result: Arc<Option<String>> = Arc::new(None);
    let consumer: StatusConsumer = StatusConsumer::new(false, result.clone());
    let args = BasicConsumeArguments::new(&queue, "");
    connection
        .chan
        .basic_consume(consumer, args)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    if let Some(json) = result.as_deref() {
        serde_json::from_str(json).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
    } else {
        Ok(None)
    }
}
