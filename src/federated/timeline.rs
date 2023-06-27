#![allow(clippy::manual_map)]
#![allow(clippy::cmp_owned)]

use std::borrow::Borrow;

use megalodon::{
    entities::{Notification, Status},
    megalodon::{
        GetAccountStatusesInputOptions, GetNotificationsInputOptions, GetTimelineOptionsWithLocal,
    },
    response::Response,
};
use regex::Regex;
use url::Url;

use crate::repository::tokens::Token;

use super::Page;

fn home_options(max_id: Option<String>) -> GetTimelineOptionsWithLocal {
    GetTimelineOptionsWithLocal {
        only_media: None,
        limit: None,
        max_id,
        since_id: None,
        min_id: None,
        local: None,
    }
}

fn notification_option(max_id: Option<String>) -> GetNotificationsInputOptions {
    GetNotificationsInputOptions {
        limit: None,
        max_id,
        since_id: None,
        min_id: None,
        exclude_types: None,
        account_id: None,
    }
}

fn account_option(max_id: Option<String>) -> GetAccountStatusesInputOptions {
    GetAccountStatusesInputOptions {
        limit: None,
        max_id,
        since_id: None,
        pinned: None,
        exclude_replies: None,
        exclude_reblogs: None,
        only_media: None,
    }
}

pub async fn get_home_timeline_page(
    token: &Token,
    max_id: Option<String>,
) -> Result<Page<Status>, String> {
    super::client::authenticated_client(token)
        .get_home_timeline(Some(&home_options(max_id.clone())))
        .await
        .map_err(|err| err.to_string())
        .map(|response| Page {
            items: response.json.clone(),
            max_id: max_id_from_response(&response),
        })
}

pub async fn get_account_timeline_page(
    account_id: String,
    account_url: String,
    max_id: Option<String>,
) -> Result<Page<Status>, String> {
    let host = Url::parse(&account_url)
        .map_err(|err| err.to_string())
        .and_then(|url| {
            if let Some(host) = url.host_str() {
                Ok(host.to_owned())
            } else {
                Err("Missing host".to_owned())
            }
        })?;
    super::client::anonymous_client(&host, None)
        .get_account_statuses(account_id, Some(&account_option(max_id.clone())))
        .await
        .map_err(|err| err.to_string())
        .map(|response| Page {
            items: response.json.clone(),
            max_id: max_id_from_response(&response),
        })
}

pub async fn get_notification_timeline_page(
    token: &Token,
    max_id: Option<String>,
) -> Result<Page<Notification>, String> {
    super::client::authenticated_client(token)
        .get_notifications(Some(&notification_option(max_id.clone())))
        .await
        .map_err(|err| err.to_string())
        .map(|response| Page {
            items: response.json(),
            max_id: max_id_from_response(&response),
        })
}

pub fn next_link(link: &str) -> Option<String> {
    if let Some(value) = link
        .split(',')
        .filter(|part| {
            part.split(';')
                .last()
                .is_some_and(|component| component.contains("rel=\"next\""))
        })
        .collect::<Vec<&str>>()
        .first()
        .copied()
    {
        Some(value.to_owned())
    } else {
        None
    }
}

pub fn get_parameter(url: &Url, parameter: &str) -> Option<String> {
    url.query_pairs().find_map(|pair| {
        if pair.0.borrow() == parameter.to_owned() {
            let inner = pair.1.as_ref().to_owned();
            Some(inner)
        } else {
            None
        }
    })
}

pub fn max_id_from_response<T>(response: &Response<T>) -> Option<String> {
    response.header["Link"]
        .to_str()
        .ok() // find the Link header if present
        .and_then(next_link) // split and get the rel=next part
        .and_then(|next| {
            // extract the url from <url>
            let re = Regex::new(r"<(.*?)>").unwrap();
            if let Some(caps) = re.captures(&next) {
                Some(caps.get(1).map_or("", |m| m.as_str()).to_owned())
            } else {
                None
            }
        })
        .and_then(|url| Url::parse(&url).ok())
        .and_then(|url| get_parameter(&url, "max_id"))
}