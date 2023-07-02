use std::{cell::RefCell, collections::HashMap};

use megalodon::{
    entities::{Context, Notification, Status},
    SNS,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::repository::tokens::Token;

use super::Page;

pub struct Response {
    /// Parsed json object.
    pub json: String,
    /// Status code of the response.
    pub status: u16,
    /// Status text of the response.
    pub status_text: String,
    /// Headers of the response.
    pub header: HashMap<String, String>,
}

struct Client {
    response: Option<Mutex<RefCell<Response>>>,
}

#[allow(non_upper_case_globals)]
static mut client: Client = Client { response: None };

pub fn test_set_mock_response<T: for<'a> Deserialize<'a> + Serialize>(value: &T) {
    unsafe {
        if client.response.is_none() {
            client.response = Some(Mutex::new(RefCell::new(Response {
                json: serde_json::to_string(value).unwrap(),
                status: 200,
                status_text: "OK".to_owned(),
                header: HashMap::new(),
            })));
        }
    }
}

async fn get<T: for<'a> Deserialize<'a>>() -> Result<T, String> {
    unsafe {
        let response = &client.response;
        let guard = &response.as_ref().unwrap().lock().await;
        let json = &guard.borrow().json;
        Ok(serde_json::from_str::<T>(json).map_err(|e| e.to_string())?)
    }
}

pub async fn get_context(
    _instance_url: &String,
    _status_id: &str,
    _sns: Option<&SNS>,
) -> Result<Option<Context>, String> {
    Ok(Some(get::<Context>().await?))
}

pub async fn resolve(_token: &Token, _status_url: &String) -> Result<Option<Status>, String> {
    Ok(Some(get::<Status>().await?))
}

pub async fn get_home_timeline_page(
    _token: &Token,
    _max_id: Option<String>,
) -> Result<Page<Status>, String> {
    let items = get::<Vec<Status>>().await?;
    Ok(Page {
        items,
        max_id: None,
    })
}

pub async fn get_account_timeline_page(
    _account_id: String,
    _account_url: String,
    _max_id: Option<String>,
) -> Result<Page<Status>, String> {
    let items = get::<Vec<Status>>().await?;
    Ok(Page {
        items,
        max_id: None,
    })
}

pub async fn get_notification_timeline_page(
    _token: &Token,
    _max_id: Option<String>,
) -> Result<Page<Notification>, String> {
    let items = get::<Vec<Notification>>().await?;
    Ok(Page {
        items,
        max_id: None,
    })
}

pub async fn get_remote_account_status_urls(
    _acct: &String,
    _limit: usize,
) -> Result<Vec<String>, String> {
    Ok(vec!["https://example.com/status/1".to_owned()])
}

pub async fn is_following(_token: &Token, _acct: &String) -> Result<bool, String> {
    Ok(false)
}
