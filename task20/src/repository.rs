use crate::models::Paste;
use std::sync::{Mutex,Arc};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Repository {
    pub urls: Arc<Mutex<HashMap<String, Paste>>>,
}

impl Repository {
    pub async fn save(&self, paste: Paste){
        self.urls.lock().unwrap().insert(paste.id.clone(), paste);
    } 

    pub async fn find(&self, id: String) -> Option<Paste>{
        let repository = self.urls.lock().unwrap();
        if let Some(paste) = repository.get(&id){
            return Some(Paste { id: id, text: paste.text.clone() });
        }
        None
    }
}