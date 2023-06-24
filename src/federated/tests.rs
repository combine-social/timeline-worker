use megalodon::response::Response;
use tokio::time::Instant;
use url::Url;

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

#[test]
fn finds_rel_next_link() {
    let link =
        "<http://example.com/?max_id=2>; rel=\"next\",<http://example.com/?min_id=2>; rel=\"prev\"";
    assert_eq!(
        super::timeline::next_link(link),
        Some("<http://example.com/?max_id=2>; rel=\"next\"".to_owned())
    );
}

#[test]
fn finds_url_parameter() {
    let url = Url::parse("http://example.com/?q=test").unwrap();

    assert_eq!(
        super::timeline::get_parameter(&url, "q"),
        Some("test".to_owned())
    );
}
