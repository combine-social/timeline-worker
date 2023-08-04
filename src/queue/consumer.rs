use amqprs::channel::{BasicAckArguments, BasicGetArguments};
use redis::FromRedisValue;

use crate::strerr::here;

use super::connect::Connection;

pub async fn consume(connection: Connection, queue: &str) -> Result<Option<String>, String> {
    if let Some(response) = connection
        .channel
        .basic_get(BasicGetArguments {
            queue: queue.to_string(),
            no_ack: false,
        })
        .await
        .map_err(|err| -> String {
            error!("Error consuming from {:?}: {:?}", queue, &err);
            here!(err)
        })?
    {
        connection
            .channel
            .basic_ack(BasicAckArguments {
                delivery_tag: response.0.delivery_tag(),
                multiple: false,
            })
            .await
            .map_err(|err| here!(err))?;
        let content = response.2;
        if let Some(payload) = String::from_byte_vec(&content) {
            debug!("payload: {:?}", payload);
            if let Some(first) = payload.first() {
                debug!("first: {:?}", first);
                Ok(Some(first.to_owned()))
            } else {
                let msg = format!("No first in payload for {:?}", &content);
                error!("{:?}", msg);
                Err(here!(msg))
            }
        } else {
            let msg = format!("Empty payload from {:?}", response.0);
            error!("{:?}", msg);
            Err(here!(msg))
        }
    } else {
        Ok(None)
    }
}
