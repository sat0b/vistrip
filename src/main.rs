#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate log;
extern crate exif;

mod config;
mod extractor;
mod indexer;
mod pexels;

use crate::config::Config;
use crate::pexels::{Downloader, PexelClient};

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::new();
    debug!("key: {}", &config.pexels_api_key);
    let client = PexelClient::new(&config.pexels_api_key);
    let downloader = Downloader::new(client, &config.download_path);
    match downloader.download().await {
        Ok(()) => {}
        Err(err) => {
            error!("failed to download: {}", err)
        }
    };
}
