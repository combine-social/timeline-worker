use async_trait::async_trait;
use megalodon::entities::Status;
#[cfg(not(test))]
use megalodon::SNS;
use reqwest::{header::ACCEPT, redirect::Policy};
use url::Url;

use crate::strerr::here;

#[cfg(not(test))]
use super::detect;

#[async_trait]
pub trait OriginId {
    async fn origin_id(&self) -> Result<String, String>;
}

#[async_trait]
impl OriginId for Status {
    #[cfg(not(test))]
    async fn origin_id(&self) -> Result<String, String> {
        let hostname = host(self)?;
        match detect::detect_sns(&hostname).await? {
            SNS::Mastodon => mastodon_origin_id(self),
            SNS::Pleroma => pleroma_origin_id(self).await,
            SNS::Friendica => friendica_origin_id(self),
            SNS::Firefish => firefish_origin_id(self),
        }
    }

    #[cfg(test)]
    async fn origin_id(&self) -> Result<String, String> {
        Ok(self.id.clone())
    }
}

fn host(status: &Status) -> Result<String, String> {
    Url::parse(&status.uri)
        .map(|url| url.host_str().map(|s| s.to_owned()))
        .map_err(|err| here!(err))
        .and_then(|opt| {
            if let Some(host) = opt {
                Ok(host)
            } else {
                Err(format!("Missing host in uri: {}", status.uri.clone()))
            }
        })
}

fn mastodon_origin_id(status: &Status) -> Result<String, String> {
    last_path_component(status.uri.clone())
}

fn firefish_origin_id(status: &Status) -> Result<String, String> {
    last_path_component(status.uri.clone())
}

async fn pleroma_origin_id(status: &Status) -> Result<String, String> {
    last_path_component(get_redirect_url(status).await?)
}

fn friendica_origin_id(status: &Status) -> Result<String, String> {
    if let Some(url) = &status.url {
        last_path_component(url.to_owned())
    } else {
        Err(format!("Missing host in url: {}", status.id.clone()))
    }
}

fn last_path_component(url: String) -> Result<String, String> {
    url.split('/')
        .last()
        .map(|s| Ok(s.to_owned()))
        .unwrap_or_else(|| {
            let msg = format!("Bad url format for {}", &url);
            Err(here!(msg))
        })
}

async fn get_redirect_url(status: &Status) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .map_err(|err| here!(err))?;
    let result = client
        .head(&status.uri)
        .header(ACCEPT, "application/json".to_owned())
        .send()
        .await;
    let response = result.unwrap();
    if let Some(location_header) = response.headers().get("Location") {
        Ok(location_header
            .to_str()
            .map_err(|err| here!(err))?
            .to_owned())
    } else {
        Err(format!("No location header from: {:?}", &status.uri))
    }
}
