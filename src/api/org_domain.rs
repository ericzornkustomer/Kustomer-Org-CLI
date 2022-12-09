use std::error::Error;

use reqwest::Client;

use crate::models::org_domain_request::OrgDomainRequest;
use crate::models::org_domain_response::OrgDomainResponse;
use super::urls::KUSTOMER_API;

pub async fn get_org_domain_data(org_name: &String) -> Result<OrgDomainResponse, Box<dyn Error>> {
    let org_name = org_name.clone();

    let req = OrgDomainRequest::new(org_name);
    let url = req.generate_url(KUSTOMER_API);

    let client = Client::new();

    let resp = client.get(url)
        .send()
        .await?
        .json::<OrgDomainResponse>()
        .await?;

    Ok(resp)
}