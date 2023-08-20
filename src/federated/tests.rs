use megalodon::response::Response;
use reqwest::header::HeaderMap;
use url::Url;

use super::timeline;
use crate::repository::registrations::Registration;
use crate::repository::tokens::Token;

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

#[tokio::test]
async fn identifies_remote_status() {
    let token = Token {
        id: 0,
        username: String::from("user"),
        access_token: String::from("token"),
        token_type: None,
        scope: None,
        created_at: None,
        fail_count: None,
        registration: Registration {
            id: 1,
            instance_url: String::from("exmaple.com"),
            registration_id: None,
            name: None,
            website: None,
            redirect_uri: String::from("https://example.com/token"),
            client_id: String::from("client"),
            client_secret: String::from("secret"),
            vapid_key: None,
            nonce: String::from("nonce"),
        },
        worker_id: 1,
    };
    assert!(
        super::resolve::is_remote(&token, &"https://remote.com/id/1".to_string())
            .is_ok_and(|remote| remote)
    );
}

#[tokio::test]
async fn identifies_local_status() {
    let token = Token {
        id: 0,
        username: String::from("user"),
        access_token: String::from("token"),
        token_type: None,
        scope: None,
        created_at: None,
        fail_count: None,
        registration: Registration {
            id: 1,
            instance_url: String::from("example.com"),
            registration_id: None,
            name: None,
            website: None,
            redirect_uri: String::from("https://example.com/token"),
            client_id: String::from("client"),
            client_secret: String::from("secret"),
            vapid_key: None,
            nonce: String::from("nonce"),
        },
        worker_id: 1,
    };
    assert!(
        !super::resolve::is_remote(&token, &"https://example.com/id/1".to_string())
            .is_ok_and(|remote| remote)
    );
}
