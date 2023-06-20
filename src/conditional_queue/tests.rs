use chrono::Utc;

use crate::{
    cache::{self, StatusCacheMetaData},
    conditional_queue,
    models::ContextRequest,
    queue::{self},
};

#[tokio::test]
async fn queues_a_status() {
    let meta = StatusCacheMetaData {
        original: "orig".to_string(),
        created_at: Utc::now(),
        index: 0,
        level: 1,
    };
    let mut cache = cache::connect().await.expect("Error connecting to cache");
    let connection = queue::connect().await.unwrap();
    let result = conditional_queue::send_if_not_cached(
        &mut cache,
        &connection,
        &"test".to_owned(),
        &"key".to_owned(),
        &"https://example.com".to_owned(),
        &"https://example.com/message/id".to_owned(),
        &"message_id".to_owned(),
        &meta,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(connection.store.borrow().keys().count(), 1);
}

#[tokio::test]
async fn skips_a_cached_status() {
    let meta = StatusCacheMetaData {
        original: "orig".to_string(),
        created_at: Utc::now(),
        index: 0,
        level: 1,
    };
    let status = ContextRequest {
        instance_url: "https://example.com".to_string(),
        status_id: "message_id".to_string(),
        status_url: "https://example.com/message/id".to_string(),
    };
    let mut cache = cache::connect().await.expect("Error connecting to cache");
    let _ = cache::set(&mut cache, &"key".to_owned(), &status, None).await;
    let connection = queue::connect().await.unwrap();
    let result = conditional_queue::send_if_not_cached(
        &mut cache,
        &connection,
        &"test".to_owned(),
        &"key".to_owned(),
        &"https://example.com".to_owned(),
        &"https://example.com/message/id".to_owned(),
        &"message_id".to_owned(),
        &meta,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(connection.store.borrow().keys().count(), 0);
}
