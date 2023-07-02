use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::get as orig_get;
use super::set as orig_set;

pub struct Cache {
    store: HashMap<String, String>,
}

pub async fn connect() -> Result<Cache, String> {
    Ok(Cache {
        store: HashMap::new(),
    })
}

pub async fn has(cache: &Cache, key: &String) -> Result<bool, String> {
    Ok(cache.store.contains_key(key))
}

pub async fn get<T>(cache: &mut Cache, key: &String) -> Result<T, String>
where
    T: for<'a> Deserialize<'a> + Sized,
{
    let value = cache.store.get(key);
    if let Some(json) = value {
        orig_get::deserialize(json)
    } else {
        Err("Missing key".to_owned())
    }
}

pub async fn set<T>(
    cache: &mut Cache,
    key: &String,
    value: &T,
    _expiry: Option<usize>,
) -> Result<(), String>
where
    T: Serialize + Sized,
{
    cache
        .store
        .insert(key.to_string(), orig_set::to_string(value)?);
    Ok(())
}

pub async fn get_keys_with_prefix(
    cache: &mut Cache,
    prefix: &String,
) -> Result<Vec<String>, String> {
    Ok(cache
        .store
        .keys()
        .map(|s| s.to_owned())
        .filter(|k| k.starts_with(prefix))
        .collect())
}

pub async fn delete_keys_with_prefix(cache: &mut Cache, prefix: &String) -> Result<(), String> {
    for key in get_keys_with_prefix(cache, prefix).await? {
        cache.store.remove(&key);
    }
    Ok(())
}
