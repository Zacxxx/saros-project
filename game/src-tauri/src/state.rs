use crate::commands::create_world::{WorldParams, WorldRecord};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
struct Inner {
    records: HashMap<String, WorldRecord>,
    params: HashMap<String, WorldParams>,
    active: Option<String>,
}

#[derive(Default, Clone)]
pub struct WorldStore(Arc<RwLock<Inner>>);

impl WorldStore {
    pub async fn insert(&self, id: String, params: WorldParams, record: WorldRecord) {
        let mut inner = self.0.write().await;
        inner.params.insert(id.clone(), params);
        inner.records.insert(id, record);
    }

    pub async fn set_active(&self, id: String) {
        self.0.write().await.active = Some(id);
    }

    pub async fn list(&self) -> Vec<WorldRecord> {
        self.0.read().await.records.values().cloned().collect()
    }

    pub async fn get_params(&self, id: &str) -> Option<WorldParams> {
        self.0.read().await.params.get(id).cloned()
    }
}
