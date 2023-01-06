use reqwest::Client;
use std::error::Error;

use crate::models::{org_data_response::OrgDataResponse, org_data_request::{OrgDataRequest}, org_domain_response::OrgDomainResponse, org_domain_request::OrgDomainRequest, shared::SharedResponse};

use super::urls::KUSTOMER_API;

pub async fn get_org_data_by_id(org_id: &String, api_key: &String) -> Result<SharedResponse, Box<dyn Error>> {
    let client = Client::new();
    
    // Make Request to Org Data Internal Endpoint with API Key
    let org_data_request = OrgDataRequest::new(org_id.clone(), api_key.clone());
    let url = org_data_request.generate_url(KUSTOMER_API);

    let org_data_resp = client.get(&url)
    .bearer_auth(org_data_request.api_key)
    .send()
    .await?
    .json::<OrgDataResponse>()
    .await?;
    
    // Make Request to Org Domain Endpoint when we have the name of the org
    let org_domain_request = OrgDomainRequest::new(org_data_resp.data.attributes.name);
    let url = org_domain_request.generate_url(KUSTOMER_API);

    let org_domain_resp = client.get(&url)
    .send()
    .await?
    .json::<OrgDomainResponse>()
    .await?;

    let final_resp = SharedResponse::new(org_domain_resp.data.attributes.org_id, org_domain_resp.data.attributes.name, Some(org_domain_resp.data.attributes.pod));

    Ok(final_resp)
}