use std::{
    collections::HashMap,
    fmt::{self},
};

use redis::RedisError;
use serde::{Deserialize, Serialize};

use super::get as orig_get;
use super::set as orig_set;

#[derive(Debug)]
struct MockError(String);

pub struct Cache {
    store: HashMap<String, String>,
}

impl std::error::Error for MockError {
    fn description(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub async fn connect() -> Result<Cache, RedisError> {
    Ok(Cache {
        store: HashMap::new(),
    })
}

pub async fn has(cache: &Cache, key: &String) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(cache.store.contains_key(key))
}

pub async fn get<T>(cache: &mut Cache, key: &String) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let value = cache.store.get(key);
    if let Some(json) = value {
        orig_get::deserialize(json)
    } else {
        Err(Box::new(MockError("Missing key".to_owned())))
    }
}

pub async fn set<T>(
    cache: &mut Cache,
    key: &String,
    value: &T,
    _expiry: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    cache
        .store
        .insert(key.to_string(), orig_set::to_string(value)?);
    Ok(())
}
