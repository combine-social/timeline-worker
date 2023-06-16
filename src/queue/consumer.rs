use std::sync::Arc;

use amqprs::{
    channel::{BasicAckArguments, Channel},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use async_trait::async_trait;
use redis::FromRedisValue;

pub struct StatusConsumer {
    no_ack: bool,
    result: Arc<Option<String>>,
}

impl StatusConsumer {
    /// Return a new consumer.
    ///
    /// See [Acknowledgement Modes](https://www.rabbitmq.com/consumers.html#acknowledgement-modes)
    ///
    /// no_ack = [`true`] means automatic ack and should NOT send ACK to server.
    ///
    /// no_ack = [`false`] means manual ack, and should send ACK message to server.

    pub fn new(no_ack: bool, result: Arc<Option<String>>) -> Self {
        Self { no_ack, result }
    }
}

#[async_trait]
impl AsyncConsumer for StatusConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        println!("in consumer");
        if let Some(payload) = String::from_byte_vec(&content) {
            println!("payload: {:?}", payload);
            if let Some(first) = payload.first() {
                println!("first: {:?}", first);
                self.result = Some(first.to_owned()).into();
            }
        };

        // ack explicitly if manual ack
        if !self.no_ack {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            let result = channel.basic_ack(args).await;
            if result.is_err() {
                println!("{:?}", result);
            }
        }
    }
}
