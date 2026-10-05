use std::path::{Path, PathBuf};
use url::Url;
use futures_util::StreamExt; // Важно: для метода .next() у потока байт
use tokio::io::AsyncWriteExt; // Важно: для .write_all() при записи в файл
use indicatif::{ProgressBar, ProgressStyle};

use crate::models::DownloadTask;

const PATH: &str = "download";

pub async fn download(mut task: DownloadTask) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = Url::parse(&task.url)?;
    println!("[СТАРТ] Скачивание: {}", url);

    // 1. Отправляем запрос
    let response = reqwest::get(url.clone()).await?;
    
    // Получаем размер файла из заголовков (если сервер его прислал)
    let total_size = response.content_length();

    // 2. Настраиваем прогресс-бар для текущего файла
    let pb = match total_size {
        Some(size) => {
            let p = ProgressBar::new(size);
            p.set_style(
                ProgressStyle::default_bar()
                    .template("{msg}\n[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                    .progress_chars("#>-")
            );
            p
        }
        None => {
            // Если размер неизвестен (например, динамический контент)
            let p = ProgressBar::new_spinner();
            p.set_style(ProgressStyle::default_spinner().template("{msg}\n{spinner} {bytes} скачано")?);
            p
        }
    };

    // Находим имя файла для вывода в прогресс-баре
    let last_segment = url.path();
    let file_name = Path::new(&last_segment).file_name().ok_or("Wrong path")?;
    let file_name_str = file_name.to_string_lossy().to_string();
    pb.set_message(format!("Скачивание: {}", file_name_str));

    // 3. Логика генерации уникального пути (ваш код без изменений)
    tokio::fs::create_dir_all(PATH).await?;
    let path = Path::new(PATH);
    task.filename = file_name_str;
    let full_path = path.join(file_name);
    let unique_file_path = generate_unique_path(&full_path).await?;

    // 4. Открываем файл для асинхронной записи
    let mut file = tokio::fs::File::create(unique_file_path).await?;
    
    // Превращаем тело ответа в поток байт (Stream)
    let mut byte_stream = response.bytes_stream();

    // 5. Читаем из сети чанками и пишем в файл, обновляя прогресс
    while let Some(chunk_result) = byte_stream.next().await {
        let chunk = chunk_result?;
        
        // Записываем кусочек в файл
        file.write_all(&chunk).await?;
        
        // Сдвигаем ползунок прогресс-бара на длину пришедшего кусочка
        pb.inc(chunk.len() as u64);
    }

    // Завершаем отображение бара
    pb.finish_with_message(format!("[ГОТОВО] Скачан: {}", task.filename));
    
    Ok(())
}


async fn generate_unique_path(path: &Path) -> Result<PathBuf, std::io::Error> {
    if !tokio::fs::try_exists(path).await? {
        return Ok(path.to_path_buf());
    }
    let dir = path.parent().unwrap_or_else(|| Path::new(""));
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|e| e.to_str());

    let mut counter = 1;

    loop {
        // Формируем новое имя: "имя (N).расширение" или просто "имя (N)"
        let new_filename = match extension {
            Some(ext) => format!("{} ({}).{}", stem, counter, ext),
            None => format!("{} ({})", stem, counter),
        };

        let new_path = dir.join(new_filename);

        if !tokio::fs::try_exists(&new_path).await? {
            return Ok(new_path);
        }

        counter += 1;
    }
}
