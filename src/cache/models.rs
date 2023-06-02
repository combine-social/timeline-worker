use redis_derive::{FromRedisValue, ToRedisArgs};

#[derive(Debug, FromRedisValue, ToRedisArgs)]
pub struct StatusCacheMetaData {
    original: String,
    created_at: String,
    index: i32,
    level: i32,
}
