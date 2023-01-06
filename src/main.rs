use std::{error::Error, env};
use clap::{Parser, Subcommand};
use colored::Colorize;

use kustomer_org::{get_org_domain_data, get_org_data_by_id, print_org_domain_response};

static KUSTOMER_PROD_API_KEY: &'static str = "KUSTOMER_PROD_API_KEY";

#[derive(Debug, Parser)]
#[command(name = "kustomer-org", about = "Kustomer Org Data Getter")]
pub struct KustomerOrgData {
    #[command(subcommand)]
    org_data_action: OrgDataAction,
}

#[derive(Debug, Subcommand)]
enum OrgDataAction {
    /// Obtains the org data within prod1 and prod2 environments by the
    /// corresponding org name.
    DataByOrgName {
        /// Finds the org by the name provided
        #[arg(short, long)]
        name: String
    },
    /// Obtains the org data within prod1 and prod2 environments by the
    /// corresponding org ID and production API key.
    DataByOrgId {
        /// Finds the org by the name provided
        #[arg(long)]
        id: String,

        /// If an API key for production is not provided, we will fallback
        /// on the KUSTOMER_PROD_API_KEY environment variable assigned on your machine
        #[arg(long)]
        api_key: Option<String>
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = KustomerOrgData::parse();

    // Run the CLI
    run(&opt).await?;
   

    Ok(())
}


async fn run(org_data: &KustomerOrgData) -> Result<(), Box<dyn Error>> {
    match org_data {
        KustomerOrgData { org_data_action } => {
            match org_data_action {
                OrgDataAction::DataByOrgName { name } => {
                    parse_by_org_name(name.to_owned()).await?
                },
                OrgDataAction::DataByOrgId { id, api_key } => {
                    parse_by_org_id(id.to_owned(), api_key.to_owned()).await?
                }
            }
            Ok(())
        }
    }
}

/// Parses data by org name passed into the CLI
async fn parse_by_org_name(org_name: String) -> Result<(), Box<dyn Error>> {
    let org_name_lower = org_name.to_lowercase();
    let org_name_upper = org_name.to_uppercase();
    let domain = get_org_domain_data(&org_name_lower).await;

    match domain {
        Ok(org_domain_data) => {
            // Output data for org
            print_org_domain_response(&org_domain_data);
        }
        Err(_e) => {
            let error_message = format!("Having trouble fetching Kustomer org data for org {}", org_name_upper).red();
            println!("{error_message}");
        }
    }

    Ok(())
}

/// Parses data by org ID and Kustomer API Key passed into the CLI
async fn parse_by_org_id(id: String, api_key: Option<String>) -> Result<(), Box<dyn Error>> {
    let finalized_api_key = api_key.clone().unwrap_or_else(|| {
        let key_from_env = env::var(KUSTOMER_PROD_API_KEY);

        match key_from_env {
            Ok(k) => k,
            Err(_) => String::new()
        }
    });

    // Handle the error case when there are no API keys specified
    if finalized_api_key.is_empty() {
        let error_message = format!("You must have an environment variable with the name KUSTOMER_PROD_API_KEY set or pass an API Key via the --api-key CLI flag").red();
        println!("{error_message}");
        return Ok(())
    }

    // Make Request for Org Data
    let res = get_org_data_by_id(&id, &finalized_api_key).await;

    match res {
        Ok(org_data_response) =>  {
            // Output data for org
            print_org_domain_response(&org_data_response);
        },
        Err(_e) => {
            let error_message = format!("Having trouble fetching Kustomer org data for org {}", id).red();
            println!("{error_message}");
        }
    }

    Ok(())
}