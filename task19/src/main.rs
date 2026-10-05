use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use axum::{Router, routing::get, routing::post};

use crate::repository::Repository;
use crate::handlers::{get_url_by_id,shorten};

mod handlers;
mod models;
mod repository;

#[derive(Clone)]
pub struct AppState {
    pub repository: Repository,
}
#[tokio::main]
async fn main() {
    let state = AppState {
        repository: Repository {
            urls: Arc::new(Mutex::new(HashMap::new())),
        },
    };

    // 1. Создаем маршрутизатор и определяем эндпоинт
    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/{id}", get(get_url_by_id))
        .with_state(state);

    // 2. Настраиваем адрес и порт для прослушивания
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!(
        "Сервер запущен на http://{}",
        listener.local_addr().unwrap()
    );

    // 3. Запускаем сервер в асинхронном рантайме Tokio
    axum::serve(listener, app).await.unwrap();
}