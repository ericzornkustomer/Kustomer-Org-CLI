use std::error::Error;
use reqwest::Client;

use crate::models::org_domain_request::OrgDomainRequest;
use crate::models::org_domain_response::OrgDomainResponse;
use crate::models::shared::SharedResponse;
use super::urls::KUSTOMER_API;

/// Obtains top level domain data for the specific org, such the org's id
/// and the name associated with that org.
pub async fn get_org_domain_data(org_name: &String) -> Result<SharedResponse, Box<dyn Error>> {
    let org_name = org_name.clone();

    let req = OrgDomainRequest::new(org_name);
    let url = req.generate_url(KUSTOMER_API);

    let client = Client::new();

    let resp = client.get(url)
        .send()
        .await?
        .json::<OrgDomainResponse>()
        .await?;
    
    let final_response = SharedResponse::new(resp.data.attributes.org_id, resp.data.attributes.name, Some(resp.data.attributes.pod));

    Ok(final_response)
}

/// TODO: Implement an API call to the Kustomer API by their Org ID. This data
/// will include a more detailed org output and will require an API key for production. 
pub async fn get_full_org_data(org_id: &String, api_key: &String) -> Result<(), Box<dyn Error>> {
    println!("{} is the ID and {} is the key", org_id, api_key);


    Ok(())
}