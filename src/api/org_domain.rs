use std::env;
use std::error::Error;
use std::process::exit;

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

pub async fn get_full_org_data(org_domain_data: &OrgDomainResponse) -> Result<(), Box<dyn Error>> {
    let kustomer_api_key = env::var("KUSTOMER_API_KEY");

    match kustomer_api_key {
        Ok(key) => println!("{} - key", key),
        Err(_e) => {
            println!("You must have KUSTOMER_API_KEY set in your environment vars");
            exit(1)
        }
    }

    Ok(())
}