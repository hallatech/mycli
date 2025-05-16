use mycli::Args;
use clap::Parser;
use std::process;
use log::{error};

fn main() {
    env_logger::init();
    let args = Args::parse();

   if let Err(e) = mycli::run(args) {
    error!("Application error: {}", e);
    process::exit(1);
   }
}
