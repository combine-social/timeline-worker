use amqprs::{
    channel::Channel,
    connection::{self, OpenConnectionArguments},
};
use std::env;

pub struct Connection {
    _connection: connection::Connection, // Hang on to connection to keep channel open
    pub channel: Channel,
}

fn url() -> String {
    env::var("QUEUE_URL").unwrap_or("amqp://localhost".to_owned())
}

pub async fn connect() -> Result<Connection, amqprs::error::Error> {
    let args = OpenConnectionArguments::try_from(url().as_str())?;
    let connection = connection::Connection::open(&args).await?;
    let channel = connection.open_channel(None).await?;
    Ok(Connection {
        _connection: connection,
        channel,
    })
}
