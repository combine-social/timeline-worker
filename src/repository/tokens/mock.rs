use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;

use super::super::registrations::{Registration, SNS};
use super::{super::mock::Connection, Token};

pub fn find_by_worker_id(con: &mut Connection, _worker_id: i32) -> impl Stream<Item = Token> + '_ {
    con
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

impl<'a> Stream for Connection {
    type Item = Token;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.taken {
            Poll::Ready(None)
        } else {
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
            };
            Poll::Ready(Some(dummy))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, None)
    }
}
