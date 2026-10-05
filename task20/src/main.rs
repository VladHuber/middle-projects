use axum::{Router, routing::get, routing::post};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::handlers::{paste, get_text};
use crate::repository::Repository;

mod handlers;
mod models;
mod repository;

#[derive(Clone)]
pub struct AppState{
    repository: Repository
}

#[tokio::main]
async fn main() {
    let state = AppState{
        repository: Repository {urls: Arc::new(Mutex::new(HashMap::new()))}
    };
    let app = Router::new()
    .route("/paste", post(paste))
    .route("/{id}", get(get_text))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

     println!(
        "Сервер запущен на http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}