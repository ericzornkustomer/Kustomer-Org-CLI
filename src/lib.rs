mod api;
mod models;
mod output;

pub use crate::api::org_domain::get_org_domain_data;
pub use crate::api::org_id::get_org_data_by_id;
pub use crate::output::print::print_org_domain_response;