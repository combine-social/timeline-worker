use tokio::time::Instant;

use crate::federated::throttle;

#[tokio::test]
async fn delays_throttled_function() {
    let mut throttle = throttle::initialize();
    let key = String::from("test");
    let first =
        throttle::throttled(&mut throttle, &key, Some(600), || async { Instant::now() }).await;
    let second =
        throttle::throttled(&mut throttle, &key, Some(600), || async { Instant::now() }).await;
    assert!(second.duration_since(first).as_secs_f64() >= 0.1);
}
