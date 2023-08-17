use megalodon::entities::Account;

use crate::{cache, federated::throttle, repository::tokens::Token, strerr::here};

pub async fn is_following(token: &Token, acct: &String) -> Result<bool, String> {
    let mut cache = cache::connect().await?;
    let key = &cache::follow_key(&token.username);
    let mut following: Vec<String> = vec![];
    if !cache::has(&mut cache, key).await? {
        info!("throttled call to verify_account_credentials");
        let account = throttle::throttled(&token.registration.instance_url, None, || async {
            let response = super::client::authenticated_client(token)
                .verify_account_credentials()
                .await
                .map_err(|err| here!(err))?;
            Ok::<Account, String>(response.json())
        })
        .await
        .map_err(|err| {
            error!("Error getting account details: {:?}", err);
            err
        })?;
        info!("throttled call to get_account_following");
        following = throttle::throttled(&token.registration.instance_url, None, || async {
            super::client::authenticated_client(token)
                .get_account_following(account.id.clone(), None)
                .await
                .map_err(|err| {
                    error!(
                        "Error getting account following for {} ({}): {:?}",
                        &account.username, &account.id, err
                    );
                    here!(err)
                })
                .map(|response| response.json())
        })
        .await
        .map_err(|err| {
            error!("Error getting following: {:?}", err);
            err
        })?
        .iter()
        .map(|followed| followed.username.clone())
        .collect();
        cache::set(&mut cache, key, &following, None).await?;
    }
    Ok(following.contains(acct))
}
