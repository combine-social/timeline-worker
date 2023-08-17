use crate::{
    federated::models::activitypub::{
        collection::OrderedCollection, page::OrderedCollectionPage, person::Person,
    },
    strerr::here,
};
use futures_util::TryFutureExt;
use reqwest::header::ACCEPT;

use super::models::activitypub::page::{ItemObject, OrderedItem};

async fn get<T>(url: &String) -> Result<T, String>
where
    T: for<'a> serde::Deserialize<'a>,
{
    let client = reqwest::Client::new();
    client
        .get(url)
        .header(ACCEPT, "application/json".to_owned())
        .send()
        .await
        .map_err(|err| format!("Error getting {}: {}", &url, here!(err)))?
        .json::<T>()
        .map_err(|err| format!("Error formatting {}: {}", &url, here!(err)))
        .await
}

fn item_url(item: OrderedItem) -> Option<String> {
    item.object.and_then(|object| match object {
        ItemObject::Create(create) => create.url,
        ItemObject::String(url) => Some(url),
        ItemObject::Object(_o) => None,
    })
}

pub async fn get_remote_account_status_urls(
    acct: &String,
    limit: usize,
) -> Result<Vec<String>, String> {
    if let Some(person_url) = webfinger::resolve(acct, true)
        .await
        .map_err(|err| {
            error!("Error resolving {}: {:?}", &acct, &err);
            format!("{:?}", err)
        })?
        .links
        .iter()
        .filter(|link| link.rel == *"self")
        .filter(|link| link.mime_type == Some("application/activity+json".to_owned()))
        .filter_map(|link| link.href.clone())
        .collect::<Vec<String>>()
        .last()
    {
        if let Some(outbox) = get::<Person>(person_url).await?.outbox {
            let outbox: OrderedCollection = get(&outbox).await?;
            let mut page_url = Some(outbox.first);
            let mut urls: Vec<String> = vec![];
            while let Some(url) = page_url {
                let page = get::<OrderedCollectionPage>(&url).await.map_err(|err| {
                    error!("Error getting page for {}: {:?}", &url, &err);
                    err
                })?;
                if let Some(items) = page.ordered_items {
                    urls = [
                        urls,
                        items
                            .iter()
                            .filter_map(|item| item_url(item.clone()))
                            .take(limit - items.len())
                            .collect(),
                    ]
                    .concat();
                }
                if urls.len() >= limit {
                    break;
                }
                page_url = page.next;
            }
            Ok(urls)
        } else {
            error!("Outbox not found for {}", person_url);
            Ok(vec![])
        }
    } else {
        error!("Account not found for {}", acct);
        Ok(vec![])
    }
}
