use chrono::Utc;

use crate::cache::{self, StatusCacheMetaData};

#[tokio::test]
async fn stores_a_value_for_a_key() {
    let meta = StatusCacheMetaData {
        original: "orig".to_owned(),
        created_at: Utc::now(),
        index: 0,
        level: 1,
    };
    let con_result = cache::connect().await;
    assert!(con_result.is_ok());
    let mut connection = con_result.unwrap();
    let result = cache::set(&mut connection, &"key".to_owned(), &meta, None).await;
    assert!(result.is_ok());
    assert!(cache::has(&connection, &"key".to_owned()).await.unwrap());
    let cached_result = cache::get(&mut connection, &"key".to_owned()).await;
    assert!(cached_result.is_ok());
    let cached: StatusCacheMetaData = cached_result.unwrap();
    assert_eq!(cached.original, meta.original);
    assert_eq!(cached.created_at.timestamp(), meta.created_at.timestamp());
    assert_eq!(cached.index, meta.index);
    assert_eq!(cached.level, meta.level);
}

#[tokio::test]
async fn deletes_keys_with_prefix() {
    let mut cache = cache::connect().await.unwrap();
    cache::set(&mut cache, &"key1".to_owned(), &"test", None)
        .await
        .unwrap();
    cache::set(&mut cache, &"key2".to_owned(), &"test", None)
        .await
        .unwrap();
    cache::set(&mut cache, &"other key".to_owned(), &"test", None)
        .await
        .unwrap();
    assert!(cache::has(&mut cache, &"key1".to_owned()).await.unwrap());
    _ = cache::delete_keys_with_prefix(&mut cache, &"key".to_owned()).await;
    assert!(!cache::has(&mut cache, &"key1".to_owned()).await.unwrap());
    assert!(cache::has(&mut cache, &"other key".to_owned())
        .await
        .unwrap());
}
