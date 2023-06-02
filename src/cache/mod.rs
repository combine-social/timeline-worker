use redis::{cmd, Client, Connection, FromRedisValue, ToRedisArgs};
use std::env;

pub mod models;

fn redis_url() -> Option<String> {
    env::var("REDIS_URL").ok()
}

fn client() -> Option<Client> {
    if let Some(url) = redis_url() {
        Client::open(url).ok()
    } else {
        None
    }
}

pub async fn establish_connection() -> Option<Connection> {
    client()?.get_connection().ok()
}

pub fn get<T: FromRedisValue>(connection: &mut Connection, key: &str) -> Option<T> {
    cmd("get").arg(key).query(connection).ok()
}

pub fn set<T: ToRedisArgs>(connection: &mut Connection, key: &str, value: &T) {
    cmd("set").arg(key).arg(value).execute(connection)
}
