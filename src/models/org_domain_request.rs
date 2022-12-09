use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct OrgDomainRequest {
    org_name: String
}

impl OrgDomainRequest {
    pub fn new(org_name: String) -> Self {
        OrgDomainRequest { org_name }
    }

    pub fn generate_url(&self, base_url: &'static str) -> String {
        format!("{}/v1/domains/{}", base_url, self.org_name)
    }
}
