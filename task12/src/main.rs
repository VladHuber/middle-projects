mod downloader;
mod service;
mod error;

use std::{env, path::Path};

// $ cargo run -- download https://github.com/ip7z/7zip/releases/download/26.02/7z2602-x64.exe
fn main() -> Result<(), Box<dyn std::error::Error>> {
     let args: Vec<String> = env::args().collect();
    // URL-адрес файла (например, изображение)
    if args.len() < 3 {
        eprintln!("Мало аргументов. Пример: cargo run -- download https://example.com/file.zip");
        return Ok(());
    }
    let path = Path::new(&args[1]);
    let url = &args[2];
    service::download_file(url, path)?;
    
    Ok(())
}
