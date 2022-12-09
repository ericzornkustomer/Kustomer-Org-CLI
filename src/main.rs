use std::error::Error;

use kustomer_org::get_org_domain_data;
use kustomer_org::get_full_org_data;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kustomer-org", about = "Kustomer Org Data Getter")]
pub struct KustomerOrgData {
    /// Obtains the org name within prod1 and prod2 environments
    #[structopt(short, long)]
    org_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = KustomerOrgData::from_args();

    match opt {
        KustomerOrgData { org_name } => {
            let domain = get_org_domain_data(&org_name).await;

            match domain {
                Ok(org_domain_data) => {
                    let org_id = org_domain_data.data.id.clone();
                    println!("{:#?} has org id - {org_id}", org_domain_data);
                    
                    // TODO: Implement
                    get_full_org_data(&org_domain_data).await?;
                }
                Err(_e) => println!("having trouble fetching Kustomer org data for org {}", org_name)
            }
        }
    }

    Ok(())
}
