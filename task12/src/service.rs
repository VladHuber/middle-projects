use std::path::Path;
use std::fs;

use crate::downloader;
use crate::error::AppResult;

pub fn download_file(url: &str, path: &Path)-> AppResult<()>{
    fs::create_dir_all(path)?;
    downloader::download(url, path)?;
    Ok(())
}