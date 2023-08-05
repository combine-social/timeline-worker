use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "@context")]
    pub context: Option<(String, String, Context)>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub following: Option<String>,
    pub followers: Option<String>,
    pub inbox: Option<String>,
    pub outbox: Option<String>,
    pub featured: Option<String>,
    pub featured_tags: Option<String>,
    pub preferred_username: Option<String>,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub manually_approves_followers: Option<bool>,
    pub discoverable: Option<bool>,
    pub published: Option<String>,
    pub devices: Option<String>,
    pub public_key: Option<PublicKey>,
    pub tag: Option<Vec<Value>>,
    pub attachment: Option<Vec<Attachment>>,
    pub endpoints: Option<Endpoints>,
    pub icon: Option<Icon>,
    pub image: Option<Image>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub manually_approves_followers: Option<String>,
    pub toot: Option<String>,
    pub featured: Option<Featured>,
    pub featured_tags: Option<FeaturedTags>,
    pub also_known_as: Option<AlsoKnownAs>,
    pub moved_to: Option<MovedTo>,
    pub schema: Option<String>,
    #[serde(rename = "PropertyValue")]
    pub property_value: Option<String>,
    pub value: Option<String>,
    pub discoverable: Option<String>,
    #[serde(rename = "Device")]
    pub device: Option<String>,
    #[serde(rename = "Ed25519Signature")]
    pub ed25519signature: Option<String>,
    #[serde(rename = "Ed25519Key")]
    pub ed25519key: Option<String>,
    #[serde(rename = "Curve25519Key")]
    pub curve25519key: Option<String>,
    #[serde(rename = "EncryptedMessage")]
    pub encrypted_message: Option<String>,
    pub public_key_base64: Option<String>,
    pub device_id: Option<String>,
    pub claim: Option<Claim>,
    pub fingerprint_key: Option<FingerprintKey>,
    pub identity_key: Option<IdentityKey>,
    pub devices: Option<Devices>,
    pub message_franking: Option<String>,
    pub message_type: Option<String>,
    pub cipher_text: Option<String>,
    pub suspended: Option<String>,
    pub focal_point: Option<FocalPoint>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Featured {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedTags {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlsoKnownAs {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovedTo {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintKey {
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityKey {
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    #[serde(rename = "@type")]
    pub type_field: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocalPoint {
    #[serde(rename = "@container")]
    pub container: Option<String>,
    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: Option<String>,
    pub owner: Option<String>,
    pub public_key_pem: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    pub shared_inbox: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub media_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub media_type: Option<String>,
    pub url: Option<String>,
}
