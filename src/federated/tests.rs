use megalodon::response::Response;
use reqwest::header::HeaderMap;
use tokio::time::Instant;
use url::Url;

use super::timeline;
use crate::federated::throttle;

#[tokio::test]
async fn delays_throttled_function() {
    let key = String::from("test");
    let first = throttle::throttled(&key, Some(600), || async { Instant::now() }).await;
    let second = throttle::throttled(&key, Some(600), || async { Instant::now() }).await;
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

#[test]
fn none_max_id_from_empty_header() {
    let res = Response {
        json: "test".to_owned(),
        status: 200,
        status_text: "OK".to_owned(),
        header: HeaderMap::new(),
    };

    assert_eq!(timeline::max_id_from_response(&res), None);
}

#[test]
fn gets_max_id_from_response() {
    let mut res = Response {
        json: "test".to_owned(),
        status: 200,
        status_text: "OK".to_owned(),
        header: HeaderMap::new(),
    };
    res.header.insert("Link", "<http://example.com/api/v1/notifications?max_id=1>; rel=\"next\", <http://example.com/api/v1/notifications?min_id=2>; rel=\"prev\"".parse().unwrap());
    assert_eq!(timeline::max_id_from_response(&res), Some("1".to_owned()));
}
