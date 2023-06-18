extern crate queues;
use std::{cell::RefCell, collections::HashMap};

use queues::*;
use serde::{Deserialize, Serialize};

use super::next as orig_next;
use super::send as orig_send;

pub struct Connection {
    store: RefCell<HashMap<String, RefCell<Queue<Vec<u8>>>>>,
}

pub async fn connect() -> Result<Connection, amqprs::error::Error> {
    Ok(Connection {
        store: RefCell::new(HashMap::new()),
    })
}

pub async fn send<T>(
    connection: &Connection,
    queue: &str,
    message: &T,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + Sized,
{
    let mut store = connection.store.borrow_mut();
    if !store.contains_key(queue) {
        store.insert(queue.to_owned(), RefCell::new(queue![]));
    }
    let mut queue = store.get(queue).unwrap().borrow_mut();
    let _ = queue.add(orig_send::into_content(message)?);
    Ok(())
}

pub async fn next<T: for<'a> Deserialize<'a> + Sized + Send + Sync>(
    connection: &Connection,
    queue: &str,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let store = connection.store.borrow();
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
}
