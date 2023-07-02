use megalodon::entities::Account;

use crate::{cache, federated::throttle, repository::tokens::Token};

pub async fn is_following(token: &Token, acct: &String) -> Result<bool, String> {
    let mut cache = cache::connect().await?;
    let key = &cache::follow_key(&token.username);
    let mut following: Vec<String> = vec![];
    if !cache::has(&mut cache, &key).await? {
        let account = throttle::throttled(&token.registration.instance_url, None, || async {
            let response = super::client::authenticated_client(&token)
                .verify_account_credentials()
                .await
                .map_err(|err| err.to_string())?;
            Ok::<Account, String>(response.json())
        })
        .await?;
        following = throttle::throttled(&token.registration.instance_url, None, || async {
            super::client::authenticated_client(&token)
                .get_account_following(account.id.clone(), None)
                .await
                .map_err(|err| err.to_string())
                .map(|response| response.json())
        })
        .await?
        .iter()
        .map(|account| account.username.clone())
        .collect();
        cache::set(&mut cache, &key, &following, None).await?;
    }
    Ok(following.contains(acct))
}
