use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_organizations::{Client};
use log::{debug, info, trace};
use crate::aws::DEFAULT_REGION;


#[tokio::main]
pub async fn list_accounts() -> Result<(), Box<dyn std::error::Error>> {
    debug!("In list_accounts");
    let region_provider = RegionProviderChain::default_provider().or_else(DEFAULT_REGION);
    debug!("Region provider: {:?}", region_provider);
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    trace!("Config: {:?}", config);
    let client = Client::new(&config);
    trace!("Client: {:?}", client);

    let resp = client.list_accounts().send().await?;
    trace!("resp: {:?}", resp);

    info!("Accounts:");
    let accounts = resp.accounts();
    // println!("{:?}",accounts);
    for account in accounts {
        println!("  {}", account.id.as_ref().unwrap());
    }

    info!("Found {} accounts", accounts.len());

    Ok(())
}
