use clap::Parser;
use std::error::Error;
use log::{info};

mod aws;
pub use crate::aws::{s3, organizations};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "aws")]
    service: String,
    #[arg(short, long, default_value = "global")]
    pub partition: String,
    #[arg(short, long)]
    pub action: String,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    info!("Service: {}", args.service);
    info!("Partition: {}", args.partition);
    info!("Action: {}", args.action);

    let _ = match args.service.as_str() {
        "aws" => {
            match args.action.as_str() {
                "list-buckets" => s3::list_buckets(),
                "list-accounts" => organizations::list_accounts(),
                _ => Err(format!("No implemention for action: {}", args.action))?,
            }
        }
        _ => Err(format!("No implemented action for service: {}", args.service))?,
    };

    Ok(())
}
