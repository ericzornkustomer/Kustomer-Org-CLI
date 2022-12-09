use colored::Colorize;

use crate::models::org_domain_response::OrgDomainResponse;

pub fn print_org_domain_response(domain_response: &OrgDomainResponse) {
    let id = domain_response.data.id.clone();
    let org_name = domain_response.data.attributes.name.clone();
    let org_name = org_name.to_uppercase();
    let pod_name = domain_response.data.attributes.pod.to_uppercase();

    let success_message = format!("{} has Org ID: {} in Pod: {}", org_name, id, pod_name).green();
    println!("{success_message}");
}