/*
POST /shorten
GET /{id}
*/
use axum::response::Redirect;
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::AppState;
use crate::models::UrlRecord;

use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};

#[derive(Deserialize)]
pub struct CreateShorten {
    url: String,
}

pub async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<CreateShorten>,
) -> (StatusCode, Json<UrlRecord>) {
    let generated_id = {
        let urls = state.repository.urls.lock().unwrap();
        loop {
            // 2. Генерируем новый короткий ID
            let id = take_shorten_url();

            // 3. Проверяем, есть ли уже такой ключ в HashMap
            if !urls.contains_key(&id) {
                break id; // ID уникален, выходим из цикла loop
            }
            // Если ID занят, цикл пойдет на следующий круг и сгенерирует новый
        }
    };
    let url_record = UrlRecord {
        id: generated_id.clone(),
        url: payload.url,
    };

    state.repository.save(&url_record).await;
    (StatusCode::CREATED, Json(url_record))
}

pub async fn get_url_by_id(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Redirect, StatusCode> {
    let url_record = state.repository.find(id).await;
    if let Some(record) = url_record {
        Ok(Redirect::temporary(&record.url))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

fn take_shorten_url() -> String {
    // Способ А: Только буквы и цифры (ASCII Alphanumeric)
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5) // Длина строки
        .map(char::from)
        .collect();

    return password;
}
