use kustomer_org::get_org_data;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "kustomer-org", about = "Kustomer Org Data Getter")]
pub struct KustomerOrgData {
    /// Obtains the org name within prod1 and prod2 environments
    #[structopt(short, long)]
    org_name: String,
}

#[tokio::main]
async fn main() {
    let opt = KustomerOrgData::from_args();

    match opt {
        KustomerOrgData { org_name } => {
            get_org_data(&org_name);
        }
    }
}
