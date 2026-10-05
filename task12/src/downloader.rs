use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::time::{Instant, Duration};
use std::thread::sleep;
use url::Url;
use reqwest::blocking::Client;

use crate::error::AppResult;

pub fn download(url: &str, path: &Path) -> AppResult<()> {
    let url = Url::parse(url)?;
    
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(60)).build()?;
    let max_attempts = 3;
    let mut response = None;

    for attempt in 1..=max_attempts{
        println!("Попытка подключения {} из {}...", attempt, max_attempts);
        match client.get(url.clone()).send(){
            Ok(res) => {
                if res.status().is_success(){
                    response = Some(res);
                    break;
                }else{
                    println!("Сервер вернул ошибку: HTTP {}", res.status());
                }
            }
            Err(err) => {
                if err.is_timeout(){
                    println!("Превышено время ожидания ответа.");
                } else {
                    println!("Ошибка сети: {}", err);
                }
            }
        }
        if attempt < max_attempts{
            println!("Повторная попытка через 3 секунды...");
            sleep(Duration::from_secs(3));
        }
    }
    
    let mut response = response.ok_or("Не удалось подключиться")?;
   
    let last_segment = url.path();
    let file_name = Path::new(&last_segment).file_name().ok_or("Wrong path")?;
    let full_path = path.join(file_name);

    let mut file = File::create(&full_path)?;
    let mut buffer = [0; 8192];

    // Проверяем, что сервер ответил успешным кодом (200 OK)

    let total_size = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok());
    
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|val| val.to_str().ok()).unwrap_or("");
    let name_file = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|val| val.to_str().ok()).unwrap_or("");
    
    println!("CONTENT_TYPE {} CONTENT_DISPOSITION {}", content_type, name_file);
    let mut downloaded: u64 = 0;
    let start_time = Instant::now();
    println!("Начало загрузки...");
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;

        // Считаем прошедшее время в секундах (минимум 0.001, чтобы избежать деления на ноль)
        let elapsed = start_time.elapsed().as_secs_f64().max(0.001);
        
        // Скорость в МБ/с (Байты / 1_048_576 / Секунды)
        let speed_mb = (downloaded as f64 / 1_048_576.0) / elapsed;


        if let Some(total) = total_size {
            let pct = (downloaded as f64 / total as f64) * 100.0;

            // Расчет оставшегося времени (ETA)
            let remaining_bytes = total.saturating_sub(downloaded);
            let speed_bytes_per_sec = downloaded as f64 / elapsed;
            
            let eta_str = if speed_bytes_per_sec > 0.0 {
                let eta_secs = (remaining_bytes as f64 / speed_bytes_per_sec).round() as u64;
                format!("{:02}:{:02}", eta_secs / 60, eta_secs % 60)
            } else {
                "--:--".to_string()
            };

            let bar_length = 30;
            let filled_length =
                ((downloaded as f64 / total as f64) * bar_length as f64).round() as usize;
            let bar = "#".repeat(filled_length) + &"-".repeat(bar_length - filled_length);
            
            
            print!(
                "\r[{}] {:.1}% ({:.2} MB / {:.2} MB) | {:.2} MB/s | ETA: {}",
                bar,
                pct,
                downloaded as f64 / 1_048_576.0,
                total as f64 / 1_048_576.0,
                speed_mb,
                eta_str
            );
        } else {
            print!("\rСкачано: {:.2} MB", downloaded as f64 / 1_048_576.0);
        }
        io::stdout().flush()?;
    }
    println!();

    // 2. Скачиваем все байты файла в память
    println!("Файл успешно скачан и сохранен в: {:?}", path);

    Ok(())
}
