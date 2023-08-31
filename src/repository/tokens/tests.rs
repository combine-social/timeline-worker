use crate::repository::{self, tokens};

#[tokio::test]
async fn finds_a_token() {
    let result = repository::create_pool().await;
    assert!(result.is_ok());
    let pool = result.unwrap();
    if let Ok(mut connection) = repository::connect(&pool).await {
        let result = tokens::find_by_worker_id(&mut connection, 1).await;
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let token = &tokens[0];
        assert_eq!(token.access_token, "token".to_owned());
    }
}
