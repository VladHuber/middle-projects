use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::UrlRecord;

#[derive(Clone)]
pub struct Repository {
    pub urls: Arc<Mutex<HashMap<String, String>>>,
}
impl Repository {
    pub async fn save(&self, record: &UrlRecord) {
        self
            .urls
            .lock()
            .unwrap()
            .insert(record.id.clone(), record.url.clone());
    }

    pub async fn find(&self, id: String) -> Option<UrlRecord> {
        let repository = self.urls.lock().unwrap();
        if let Some(rep) = repository.get(&id) {
            return Some(UrlRecord {
                id: id.to_string(),
                url: rep.to_string(),
            });
        }
        None
    }
}
