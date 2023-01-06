pub struct SharedResponse {
    pub org_id: String,
    pub org_name: String,
    pub pod: Option<String>
}

impl SharedResponse {
    pub fn new(org_id: String, org_name: String, pod: Option<String>) -> Self {
        SharedResponse { org_id: org_id, org_name: org_name, pod }
    }
}