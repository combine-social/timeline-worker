use super::super::registrations::{Registration, SNS};
use super::{super::mock::Connection, Token};

pub async fn find_by_worker_id(
    _con: &mut Connection,
    _worker_id: i32,
) -> Result<Vec<Token>, String> {
    let dummy = Token {
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
            sns: Some(SNS::Mastodon),
        },
        worker_id: 1,
        ping_at: None,
    };
    Ok(vec![dummy])
}

pub async fn update_fail_count(
    _con: &mut Connection,
    _token: &Token,
    _count: i32,
) -> Result<(), String> {
    Ok(())
}

pub async fn delete(_con: &mut Connection, _token: &Token) -> Result<(), String> {
    Ok(())
}
