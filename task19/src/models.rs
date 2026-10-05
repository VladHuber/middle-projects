use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct UrlRecord {
    pub id: String,
    pub url: String,
}