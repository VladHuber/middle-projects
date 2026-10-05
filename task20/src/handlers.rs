use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};

use axum::{Json, extract::Path, extract::State, http::StatusCode};

use serde::Deserialize;

use crate::AppState;
use crate::models::Paste;

#[derive(Deserialize)]
pub struct CreatePaste {
    text: String,
}
pub async fn paste(
    State(state): State<AppState>,
    Json(payload): Json<CreatePaste>,
) -> (StatusCode, Json<Paste>) {
    let generated_id = {
        let urls = state.repository.urls.lock().unwrap();
        loop {
            let id = take_random_path();
            if !urls.contains_key(&id) {
                break id;
            }
        }
    };
    let paste = Paste {
        id: generated_id.clone(),
        text: payload.text,
    };
    state.repository.save(paste.clone()).await;
    (StatusCode::CREATED, Json(paste))
}
pub async fn get_text(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    let paste = state.repository.find(id).await;
    if let Some(paste) = paste{
        Ok(paste.text)
    }else{
        Err(StatusCode::NOT_FOUND)
    }
}
fn take_random_path() -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(5) // Длина строки
        .map(char::from)
        .collect();

    return password;
}
