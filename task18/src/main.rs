use axum::{
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;


#[derive(Deserialize)]
struct DeleteUser {
    id: u64,
}

// Структура для входящих данных (десериализация из JSON)
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// Структура для ответа (сериализация в JSON)
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    // Настройка маршрутов (роутера)
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users", delete(delete_user));

    // Определение адреса сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущен на http://{}", addr);

    // Запуск сервера
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Простой GET обработчик
async fn root() -> &'static str {
    "Hello, World!"
}

// POST обработчик, принимающий и возвращающий JSON
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    Json(user)
}

async fn delete_user(Json(payload): Json<DeleteUser>) -> Json<User> {
    let user = User {
        id: payload.id,
        username: "Deleted".to_string(),
    };
    Json(user)
}