use serde::{Serialize, Deserialize};

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgDomainResponse {
    #[serde(rename = "data")]
    pub data: OrgDomainData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgDomainData {
    #[serde(rename = "type")]
    pub data_type: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "attributes")]
    pub attributes: Attributes,

    #[serde(rename = "links")]
    pub links: Links,

    #[serde(rename = "relationships")]
    pub relationships: Relationships,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "orgId")]
    pub org_id: String,

    #[serde(rename = "reservation")]
    pub reservation: bool,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "pod")]
    pub pod: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    #[serde(rename = "org")]
    pub org: Org,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Org {
    #[serde(rename = "links")]
    pub links: Links,

    #[serde(rename = "data")]
    pub data: OrgData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgData {
    #[serde(rename = "type")]
    pub data_type: String,

    #[serde(rename = "id")]
    pub id: String,
}
