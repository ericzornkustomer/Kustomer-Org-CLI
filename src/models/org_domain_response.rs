use serde::{Serialize, Deserialize};

extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgDomainResponse {
    #[serde(rename = "data")]
    data: OrgDomainData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgDomainData {
    #[serde(rename = "type")]
    data_type: String,

    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "attributes")]
    attributes: Attributes,

    #[serde(rename = "links")]
    links: Links,

    #[serde(rename = "relationships")]
    relationships: Relationships,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "orgId")]
    org_id: String,

    #[serde(rename = "reservation")]
    reservation: bool,

    #[serde(rename = "updatedAt")]
    updated_at: String,

    #[serde(rename = "createdAt")]
    created_at: String,

    #[serde(rename = "pod")]
    pod: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    #[serde(rename = "org")]
    org: Org,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Org {
    #[serde(rename = "links")]
    links: Links,

    #[serde(rename = "data")]
    data: OrgData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgData {
    #[serde(rename = "type")]
    data_type: String,

    #[serde(rename = "id")]
    id: String,
}
