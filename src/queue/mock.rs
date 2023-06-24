extern crate queues;
use std::{cell::RefCell, collections::HashMap};

use queues::*;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::next as orig_next;
use super::send as orig_send;

pub struct Connection {
    pub store: Option<Mutex<RefCell<HashMap<String, RefCell<Queue<Vec<u8>>>>>>>,
}

#[allow(non_upper_case_globals)]
static mut connection: Connection = Connection { store: None };

fn connect() {
    unsafe {
        if connection.store.is_none() {
            connection.store = Some(Mutex::new(RefCell::new(HashMap::new())));
        }
    }
}

pub async fn send<T>(queue: &str, message: &T) -> Result<(), String>
where
    T: Serialize + Sized,
{
    connect();
    unsafe {
        if let Some(mutex) = &connection.store {
            let rc = mutex.lock().await;
            let mut store = (*rc).borrow_mut();
            if !store.contains_key(queue) {
                store.insert(queue.to_owned(), RefCell::new(queue![]));
            }
            let mut queue = store.get(queue).unwrap().borrow_mut();
            let result = orig_send::into_content(message);
            if let Ok(content) = result {
                let _ = queue.add(content);
                Ok(())
            } else {
                Err(result.err().unwrap().to_string())
            }
        } else {
            Err("No store".to_string())
        }
    }
}

pub async fn next<T: for<'a> Deserialize<'a> + Sized + Send + Sync>(
    queue: &str,
) -> Result<Option<T>, String> {
    connect();
    unsafe {
        if let Some(mutex) = &connection.store {
            let rc = mutex.lock().await;
            let store = (*rc).borrow();
            if !store.contains_key(queue) {
                return Ok(None);
            }
            let mut queue = store.get(queue).unwrap().borrow_mut();
            if let Ok(vec) = queue.remove() {
                orig_next::into(
                    String::from_utf8(vec)
                        .expect("Invalid queued data")
                        .as_str(),
                )
            } else {
                Ok(None)
            }
        } else {
            Err("No store".to_string())
        }
    }
}

pub async fn size(queue_name: &str) -> Result<u32, String> {
    connect();
    connect();
    unsafe {
        if let Some(mutex) = &connection.store {
            let rc = mutex.lock().await;
            let store = (*rc).borrow();
            if !store.contains_key(queue_name) {
                return Ok(0);
            }
            let queue = store.get(queue_name).unwrap().borrow();
            Ok(queue.size() as u32)
        } else {
            Err("No store".to_string())
        }
    }
}
