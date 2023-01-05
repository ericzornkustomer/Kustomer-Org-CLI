use colored::Colorize;

use crate::models::{shared::SharedResponse};

pub fn print_org_domain_response(shared_response: &SharedResponse) {
    let org_id = shared_response.org_id.clone();
    let org_name = shared_response.org_name.to_uppercase();

    let mut pod_name = String::from("not found");

    if let Some(pod) = &shared_response.pod {
        pod_name = pod.clone();
    }


    let success_message = format!("{} has Org ID: {} in Pod: {}", org_name, org_id, pod_name).green();
    println!("{success_message}");
}