use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use crate::state::WorldStore;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorldParams {
    pub name: String,
    pub seed: u64,
    pub size: u32,
    pub sea_level: u32,
    pub cave_density: f32,
    pub structure_frequency: f32,
    pub biome_weights: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct WorldRecord {
    pub id: String,
    pub name: String,
    pub seed: u64,
    pub created_at: String,
}

#[tauri::command]
pub async fn create_world(
    params: WorldParams,
    store: State<'_, WorldStore>,
) -> Result<WorldRecord, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let record = WorldRecord {
        id: id.clone(),
        name: params.name.clone(),
        seed: params.seed,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    store.insert(id, params, record.clone()).await;
    Ok(record)
}
