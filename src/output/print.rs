use colored::Colorize;

use crate::models::org_domain_response::OrgDomainResponse;

pub fn print_org_domain_response(domain_response: &OrgDomainResponse) {
    let id = domain_response.data.id.clone();
    let org_name = domain_response.data.attributes.name.clone();

    let success_message = format!("{} has org id - {}", org_name.to_uppercase(), id).green();
    println!("{success_message}");
}