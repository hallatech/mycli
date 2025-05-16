use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client};
use log::{debug, info, trace};
use crate::aws::DEFAULT_REGION;


#[tokio::main]
pub async fn list_buckets() -> Result<(), Box<dyn std::error::Error>> {
    debug!("In list_buckets");
    let region_provider = RegionProviderChain::default_provider().or_else(DEFAULT_REGION);
    debug!("Region provider: {:?}", region_provider);
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    trace!("Config: {:?}", config);
    let client = Client::new(&config);
    trace!("Client: {:?}", client);

    let resp = client.list_buckets().send().await?;
    trace!("resp: {:?}", resp);

    info!("Buckets:");
    let buckets = resp.buckets();
    // println!("{:?}",buckets);
    for bucket in buckets {
        println!("  {}", bucket.name.as_ref().unwrap());
    }

    info!("Found {} buckets", buckets.len());

    Ok(())
}
