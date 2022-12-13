use std::error::Error;
use reqwest::Client;

use crate::models::org_domain_request::OrgDomainRequest;
use crate::models::org_domain_response::OrgDomainResponse;
use super::urls::KUSTOMER_API;

/// Obtains top level domain data for the specific org, such the org's id
/// and the name associated with that org.
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

/// TODO: Implement an API call to the Kustomer API by their Org ID. This data
/// will include a more detailed org output and will require an API key for production. 
pub async fn get_full_org_data(_org_domain_data: &OrgDomainResponse) -> Result<(), Box<dyn Error>> {
    // let kustomer_api_key = env::var("KUSTOMER_API_KEY")
    print!("");

    Ok(())
}