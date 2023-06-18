use redis::aio::Connection;
use redis::AsyncCommands;
use serde::Deserialize;

pub async fn has(
    connection: &mut Connection,
    key: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let exists: u32 = connection.exists(key).await?;
    Ok(exists != 0)
}

async fn get_string(
    connection: &mut Connection,
    key: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    connection
        .get(key)
        .await
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}

pub async fn get<T>(
    connection: &mut Connection,
    key: &String,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let json = get_string(connection, key).await?;
    serde_json::from_str(&json).map_err(|e| -> Box<dyn std::error::Error> { e.into() })
}
