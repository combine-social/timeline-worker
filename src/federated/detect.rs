use std::str::FromStr;

use megalodon::{self, SNS};

use crate::{cache, strerr::here};

// This means that instance detection runs once per day, which should be rare enough to not
// be a bottleneck.
// Should an admin replace the fediverse software on an instance with somthing incompatible,
// then SNS will only be wrong for a day, and then be re-detected.
const EXPIRY: usize = 24 * 60 * 60;

pub async fn detect_sns(instance_url: &str) -> Result<SNS, String> {
    let mut cache = cache::connect().await?;
    let key = cache::sns_key(&instance_url.to_string());
    if !cache::has(&mut cache, &key).await? {
        let url = format!("https://{}", instance_url);
        let sns = megalodon::detector(url.as_str())
            .await
            .map_err(|err| here!(err))?;
        cache::set(&mut cache, &key, &sns.to_string(), Some(EXPIRY)).await?;
        Ok(sns)
    } else {
        let sns: String = cache::get(&mut cache, &key).await?;
        SNS::from_str(sns.as_str())
    }
}
