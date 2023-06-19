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
}
