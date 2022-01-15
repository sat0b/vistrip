use anyhow::{format_err, Context, Result};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};

pub struct PexelClient {
    pexels_api_key: String,
}

impl PexelClient {
    pub fn new(pexels_api_key: &str) -> Self {
        PexelClient {
            pexels_api_key: pexels_api_key.into(),
        }
    }

    async fn get(&self, id: u32, url: &str) -> Result<()> {
        let image_path = format!("images/{}.jpg", id);
        let res = reqwest::get(url).await?;
        let bytes = &res.bytes().await?;
        let mut cursor = io::Cursor::new(bytes);
        let mut out = File::create(image_path).context("failed to create file")?;
        io::copy(&mut cursor, &mut out)?;
        debug!("download {}", url);
        Ok(())
    }

    async fn search(&self, keyword: &str) -> Result<SearchResponse> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.pexels.com/v1/search")
            .header(reqwest::header::AUTHORIZATION, &self.pexels_api_key)
            .query(&[("query", keyword), ("orientation", "landscape")])
            .send()
            .await?;
        match res.status() {
            StatusCode::OK => Ok(res.json::<SearchResponse>().await?),
            code => Err(format_err!("failed to request: {}", code)),
        }
    }
}

pub struct Downloader {
    client: PexelClient,
    download_path: PathBuf,
}

impl Downloader {
    pub fn new(client: PexelClient, download_path: &str) -> Self {
        Downloader {
            client,
            download_path: download_path.into(),
        }
    }

    pub async fn download(&self) -> Result<()> {
        self.init_directory()?;
        let response = self.client.search("japan").await.context("failed search")?;
        for photo in response.photos {
            self.client.get(photo.id, &photo.src.original).await?;
        }
        Ok(())
    }

    fn init_directory(&self) -> Result<()> {
        if !self.download_path.exists() {
            info!("create image directory");
            fs::create_dir(&self.download_path)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PhotoSrc {
    original: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Photo {
    id: u32,
    src: PhotoSrc,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    total_results: u32,
    page: u32,
    per_page: u32,
    photos: Vec<Photo>,
}
