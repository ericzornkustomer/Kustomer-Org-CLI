use std::error::Error;
use clap::Parser;
use colored::Colorize;

use kustomer_org::{get_org_domain_data, get_full_org_data, print_org_domain_response};

#[derive(Debug, Parser)]
#[command(name = "kustomer-org", about = "Kustomer Org Data Getter")]
pub struct KustomerOrgData {
    /// Obtains the org data within prod1 and prod2 environments by the
    /// corresponding org name.
    #[arg(short, long)]
    org_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = KustomerOrgData::parse();

    match opt {
        KustomerOrgData { org_name } => {
            let org_name_lower = org_name.to_lowercase();
            let org_name_upper = org_name.to_uppercase();
            let domain = get_org_domain_data(&org_name_lower).await;

            match domain {
                Ok(org_domain_data) => {
                    // Output data for org
                    print_org_domain_response(&org_domain_data);
                    get_full_org_data(&org_domain_data).await?;
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
