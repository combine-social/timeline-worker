use futures_util::StreamExt;

use crate::repository::{self, tokens};

#[tokio::test]
async fn finds_a_token() {
    let result = repository::create_pool().await;
    assert!(result.is_ok());
    let pool = result.unwrap();
    if let Ok(mut connection) = repository::connect(&pool).await {
        let mut tokens = tokens::find_all(&mut connection);
        let next = tokens.next().await;
        assert!(next.is_some());
        let token = next.unwrap();
        assert_eq!(token.access_token, "token".to_owned());
    }
}
