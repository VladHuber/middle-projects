use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Paste {
    pub id: String,
    pub text: String,
}