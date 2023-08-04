use crate::strerr::here;

use super::{
    connect,
    consumer::{self},
    declare,
};
use serde::Deserialize;
use std::any::type_name;

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
        declare::queue_declare(&connection.channel, queue)
            .await
            .map_err(|err| {
                error!("Error declaring queue: {:?}", err);
                err
            })?;
        if let Some(content) = consumer::consume(connection, queue).await? {
            into(&content).or_else(|err| {
                error!("into {:?} failed: {:?}", type_name::<T>(), &err);
                Err(here!(err))
            })
        } else {
            Ok(None)
        }
    } else {
        Err(result.err().unwrap().to_string())
    }
}
