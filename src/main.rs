mod config;
mod bot;
mod database;

use crate::config::Config;
use simple_logger::SimpleLogger;
use log::{
	LevelFilter, 
	info, 
	warn,
};

#[tokio::main]
async fn main() {
	SimpleLogger::new()
        .with_level(LevelFilter::Info)
		.init()
		.unwrap();
	
	info!("Starting...");
    
    let config = Config::new();

    if config.token.is_empty() {
        warn!("Please fill out the config.yml");
        return;
    }

    bot::start(config).await;
}
