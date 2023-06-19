use megalodon::Megalodon;

use crate::repository::tokens::Token;

pub fn authenticated_client(token: &Token) -> Box<dyn Megalodon + Send + Sync> {
    megalodon::generator(
        megalodon::SNS::Mastodon,
        String::from(format!("https://{}", token.registration.instance_url)),
        Some(token.access_token.clone()),
        None,
    )
}
