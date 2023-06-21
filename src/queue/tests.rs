use crate::{
    models::ContextRequest,
    queue::{self},
};

#[tokio::test]
async fn queues_a_status() {
    let status = ContextRequest {
        instance_url: "http://example.com".to_owned(),
        status_url: "http://example.com/message/id".to_owned(),
        status_id: "id".to_owned(),
    };
    let result = queue::send("test", &status).await;
    assert!(result.is_ok());
    let next = queue::next("test").await;
    assert!(next.is_ok());
    let queued: Option<ContextRequest> = next.ok().unwrap();
    assert!(queued.is_some());
    if let Some(queued_status) = queued {
        assert_eq!(queued_status.instance_url, status.instance_url);
    } else {
        panic!();
    }
}
