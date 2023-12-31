use crate::strerr::here;

use super::{
    connect,
    consumer::{self},
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
        if let Some(content) = consumer::consume(connection, queue).await? {
            into(&content).map_err(|err| {
                error!("into {:?} failed: {:?}", type_name::<T>(), &err);
                here!(err)
            })
        } else {
            Ok(None)
        }
    } else {
        Err(result.err().unwrap().to_string())
    }
}
