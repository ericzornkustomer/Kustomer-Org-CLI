pub struct OrgDataRequest {
    pub org_id: String,
    pub api_key: String
}

impl OrgDataRequest {
    pub fn new(org_id: String, api_key: String) -> Self {
        OrgDataRequest { org_id, api_key }
    } 
    
    pub fn generate_url(&self, base_url: &'static str) -> String {
        format!("{}/v1/orgs/{}", base_url, self.org_id)
    }
}