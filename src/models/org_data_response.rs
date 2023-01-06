use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OrgDataResponse {
    #[serde(rename = "data")]
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub data_type: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "attributes")]
    pub attributes: Attributes,

    #[serde(rename = "links")]
    pub links: Links,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "meta")]
    pub meta: Meta,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "registeredIpAddress")]
    pub registered_ip_address: String,

    #[serde(rename = "registeredExternal")]
    pub registered_external: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: String,
}
