use std::{env, error::Error};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::sync::Arc;
use tokio::sync::Semaphore;


use crate::downloader::download;
use crate::models::DownloadTask;

mod downloader;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Мало аргументов");
        return Ok(());
    }
    let url = format!("assets/{}", args[1]);
    let file = File::open(url).await?;
    let mut buffer_reader = BufReader::new(file);
    let mut line = String::new();

    let mut handles = Vec::new();
    let semaphore = Arc::new(Semaphore::new(5));
    while buffer_reader.read_line(&mut line).await? > 0 {
        let url_string = line.trim().to_string();
        if url_string.is_empty() {
            line.clear();
            continue;
        }
        let task = DownloadTask {
            url: url_string,
            filename: String::new(),
        };
        let sem = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.unwrap();
            
            if let Err(e) = download(task).await {
                eprintln!("Ошибка загрузки: {}", e);
            }
        });

        handles.push(handle);

        line.clear();
    }
    for handle in handles {
        handle.await?;
    }
    Ok(())
}
