#[derive(Default, Clone, Debug)]
pub struct Config {
    pub pexels_api_key: String,
    pub download_path: String,
}

impl Config {
    pub fn new() -> Self {
        let pexels_api_key = dotenv!("PEXELS_API_KEY");
        let download_path = dotenv!("DOWNLOAD_PATH");
        Config {
            pexels_api_key: pexels_api_key.to_string(),
            download_path: download_path.to_string(),
        }
    }
}
