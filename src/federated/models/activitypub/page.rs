use std::collections::BTreeMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub part_of: Option<String>,
    pub ordered_items: Option<Vec<OrderedItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedItem {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub actor: Option<String>,
    pub published: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub object: Option<ItemObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemObject {
    Create(CreateItemObject),
    Object(BTreeMap<String, serde_json::Value>),
    String(String),
}

impl Default for ItemObject {
    fn default() -> ItemObject {
        ItemObject::String("".to_owned())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateItemObject {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub summary: Option<Value>,
    pub in_reply_to: Option<Value>,
    pub published: Option<String>,
    pub url: Option<String>,
    pub attributed_to: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub sensitive: Option<bool>,
    pub atom_uri: Option<String>,
    pub in_reply_to_atom_uri: Option<Value>,
    pub conversation: Option<String>,
    pub local_only: Option<bool>,
    pub content: Option<String>,
    pub content_map: Option<ContentMap>,
    pub attachment: Option<Vec<Attachment>>,
    pub tag: Option<Vec<Tag>>,
    pub replies: Option<Replies>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentMap {
    pub en: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub media_type: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub blurhash: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub href: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replies {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub first: Option<First>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct First {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub next: Option<String>,
    pub part_of: Option<String>,
    pub items: Option<Vec<Value>>,
}
