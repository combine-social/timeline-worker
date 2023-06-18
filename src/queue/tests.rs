use crate::queue::{self, QueuedStatus};

#[tokio::test]
async fn queues_a_status() {
    let status = QueuedStatus {
        instance_url: "http://example.com".to_owned(),
        status_url: "http://example.com".to_owned(),
    };
    let connection = queue::connect().await.unwrap();
    let result = queue::send(&connection, "test", &status).await;
    assert!(result.is_ok());
    let next = queue::next(&connection, "test").await;
    assert!(next.is_ok());
    let queued: Option<QueuedStatus> = next.ok().unwrap();
    assert!(queued.is_some());
    if let Some(queued_status) = queued {
        assert_eq!(queued_status.instance_url, status.instance_url);
    } else {
        panic!();
    }
}
