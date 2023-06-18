use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Stream;

use super::super::registrations::Registration;
use super::{super::mock::Connection, Token};

pub fn find_all(con: &mut Connection) -> impl Stream<Item = Token> + '_ {
    con
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
                    instance_url: String::from("https://exmaple.com"),
                    registration_id: None,
                    name: None,
                    website: None,
                    redirect_uri: String::from("https://example.com/token"),
                    client_id: String::from("client"),
                    client_secret: String::from("secret"),
                    vapid_key: None,
                    nonce: String::from("nonce"),
                },
            };
            Poll::Ready(Some(dummy))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, None)
    }
}
