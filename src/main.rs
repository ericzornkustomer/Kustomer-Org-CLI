use std::error::Error;
use structopt::StructOpt;
use colored::Colorize;

use kustomer_org::get_org_domain_data;
// use kustomer_org::get_full_org_data;

#[derive(Debug, StructOpt)]
#[structopt(name = "kustomer-org", about = "Kustomer Org Data Getter")]
pub struct KustomerOrgData {
    /// Obtains the org data within prod1 and prod2 environments by the
    /// corresponding org name.
    #[structopt(short, long)]
    org_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = KustomerOrgData::from_args();

    match opt {
        KustomerOrgData { org_name } => {
            let org_name_lower = org_name.to_lowercase();
            let org_name_upper = org_name.to_uppercase();
            let domain = get_org_domain_data(&org_name_lower).await;

            match domain {
                Ok(org_domain_data) => {
                    let id = org_domain_data.data.id;

                    let success_message = format!("{} has org id - {}", org_name_upper, id).green();
                    println!("{success_message}");
                    
                    // TODO: Implement later for more rich 
                    // get_full_org_data(&org_domain_data).await?;
                }
                Err(_e) => {
                    let error_message = format!("Having trouble fetching Kustomer org data for org {}", org_name_upper).red();
                    println!("{error_message}");
                }
            }
        }
    }

    Ok(())
}
