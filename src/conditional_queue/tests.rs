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
    let send_result = conditional_queue::send_if_not_cached(
        &mut cache,
        &"test".to_owned(),
        &"key".to_owned(),
        &ContextRequest {
            instance_url: "https://example.com".to_owned(),
            status_id: "message_id".to_owned(),
            status_url: "https://example.com/message/id".to_owned(),
        },
        &meta,
    )
    .await;
    assert!(send_result.is_ok());
    let next_result: Result<Option<ContextRequest>, String> = queue::next(&"test".to_owned()).await;
    assert!(next_result.is_ok());
    assert!(next_result.ok().is_some_and(|x| x.is_some()));
}

#[tokio::test]
async fn skips_a_cached_status() {
    let meta = StatusCacheMetaData {
        original: "orig".to_string(),
        created_at: Utc::now(),
        index: 0,
        level: 1,
    };
    let request = ContextRequest {
        instance_url: "https://example.com".to_string(),
        status_id: "message_id".to_string(),
        status_url: "https://example.com/message/id".to_string(),
    };
    let mut cache = cache::connect().await.expect("Error connecting to cache");
    let _ = cache::set(&mut cache, &"key".to_owned(), &request, None).await;
    assert!(cache::has(&cache, &"key".to_owned()).await.ok().unwrap());
    let send_result = conditional_queue::send_if_not_cached(
        &mut cache,
        &"test".to_owned(),
        &"key".to_owned(),
        &request,
        &meta,
    )
    .await;
    assert!(send_result.is_ok());
    let next_result: Result<Option<ContextRequest>, String> = queue::next(&"test".to_owned()).await;
    assert!(next_result.is_ok());
    assert!(next_result.ok().is_some_and(|x| x.is_none()));
}
