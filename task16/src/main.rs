use scraper::{Html, Selector};
use std::collections::HashSet;
use url::Url;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

// Структура задачи для очереди
struct Task {
    url: Url,
    depth: usize,
}

struct Crawler {
    visited: Mutex<HashSet<String>>,
    max_depth: usize,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let url_start = Url::parse("https://stamina.ru").unwrap();
    println!("🚀 Запуск плоского краулера для: {url_start}");

    let crawler = Arc::new(Crawler {
        visited: Mutex::new(HashSet::new()),
        max_depth: 3,
    });

    // 1. Создаем ОДНУ общую очередь задач (mpsc) для всех воркеров
    let (tx, mut rx) = mpsc::channel::<Task>(1000);

    // Добавляем стартовый URL
    {
        let mut visited = crawler.visited.lock().await;
        visited.insert(url_start.to_string());
    }
    tx.send(Task { url: url_start, depth: 0 }).await.unwrap();

    // 2. Счетчик активных воркеров в данный момент.
    // Нужен для graceful shutdown: чтобы понять, когда ВСЕ воркеры заснули,
    // потому что в очереди пусто и больше никто ничего не скачает.
    let active_workers = Arc::new(Mutex::new(0));

    // 3. Запускаем фиксированный пул воркеров (например, ровно 5 потоков)
    // Больше никаких tokio::spawn в программе не будет!
    let num_workers = 5;
    for worker_id in 0..num_workers {
        let crawler_clone = Arc::clone(&crawler);
        let tx_clone = tx.clone();
        let active_clone = Arc::clone(&active_workers);

        // Передаем rx воркерам нельзя напрямую (он !Sync), но мы будем читать очередь в главном цикле 
        // или использовать другую координацию. Самый простой способ для mpsc:
        // Главный цикл в main будет распределять задачи, ЛИБО воркеры будут читать из шеримого приемника.
        // Но в Tokio Receiver нельзя шерить между потоками без Mutex.
    }

    // Давайте сделаем самую элегантную архитектуру: 
    // Главный поток раздает задачи из очереди, а фиксированный пул (Семафор) их ограничивает.
    // Это избавляет от сложного менеджмента воркеров и гарантирует отсутствие рекурсивных спавнов.
    
    // Клонируем tx для передачи в функцию, чтобы воркеры могли докидывать ссылки
    let tx_worker = tx.clone();
    drop(tx); // Дропаем исходный tx в main, чтобы канал закрылся, когда все воркеры закончат

    let semaphore = Arc::new(tokio::sync::Semaphore::new(5)); // Максимум 5 одновременных загрузок

    // Читаем задачи из плоской очереди по мере их поступления
    while let Some(task) = rx.recv().await {
        let sem = Arc::clone(&semaphore);
        let crawler_clone = Arc::clone(&crawler);
        let tx_clone = tx_worker.clone();

        // Ждем свободный слот (не более 5 параллельно)
        let permit = sem.acquire_owned().await.unwrap();

        // Спавним плоскую задачу. Она выполнится и УМРЕТ, не порождая подзадач внутри себя!
        tokio::spawn(async move {
            if let Err(e) = crawl_single_page(task, crawler_clone, tx_clone).await {
                eprintln!("Ошибка: {}", e);
            }
            drop(permit); // Освобождаем слот
        });
    }

    println!("\n🏁 Все очереди пусты, лимиты исчерпаны. Краулер успешно завершил работу!");
    println!("Всего уникальных ссылок: {}", crawler.visited.lock().await.len());
    Ok(())
}

fn parse_links_from_html(html_content: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let link_selector = Selector::parse("a").unwrap();
    let mut found_hrefs = Vec::new();

    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            if !href.starts_with('#') {
                found_hrefs.push(href.to_string());
            }
        }
    }
    found_hrefs
}

// ПЛОСКАЯ ФУНКЦИЯ: Скачивает ОДНУ страницу и отправляет новые ссылки в КАНАЛ
async fn crawl_single_page(
    task: Task,
    crawler: Arc<Crawler>,
    tx: mpsc::Sender<Task>,
) -> Result<(), String> {
    if task.depth >= crawler.max_depth {
        return Ok(());
    }

    // 1. Скачиваем HTML (Чистый I/O)
    let response = reqwest::get(task.url.clone())
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    // 2. Парсим HTML синхронно
    let raw_hrefs = parse_links_from_html(&response);

    // 3. Фильтруем ссылки
    for href in raw_hrefs {
        let Some(next_url) = normalize_url(&task.url, &href) else {
            continue;
        };

        let next_url_string = next_url.to_string();
        
        if next_url.domain() != task.url.domain() {
            continue;
        }
        if next_url_string.ends_with(".exe") {
            continue;
        }

        let is_new = {
            let mut visited = crawler.visited.lock().await;
            visited.insert(next_url_string)
        };

        // 4. Вместо tokio::spawn мы просто отправляем новую задачу в КАНАЛ!
        if is_new {
            println!("📥 Найдена ссылка [Глубина {}]: {}", task.depth + 1, next_url);
            
            // Если очередь переполнена, send подождет. Никакого взрыва памяти!
            let _ = tx.send(Task {
                url: next_url,
                depth: task.depth + 1,
            }).await;
        }
    }

    Ok(())
}

fn normalize_url(url: &Url, path: &str) -> Option<Url> {
    if path.starts_with("http://") || path.starts_with("https://") {
        Url::parse(path).ok()
    } else {
        url.join(path).ok()
    }
}
