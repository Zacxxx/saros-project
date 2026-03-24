use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HotbarSlot {
    pub item_id: u32,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CombatStateSnapshot {
    pub hp: i32,
    pub max_hp: i32,
    pub ap: u32,
    pub max_ap: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub round: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NpcSnapshot {
    pub id: u64,
    pub name: String,
    pub traits: Vec<String>,
    pub memory: Vec<(String, String)>,
    pub current_thought: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProgress {
    pub percent: f32,
    pub status: String,
    pub done: bool,
}

impl Default for GenerationProgress {
    fn default() -> Self {
        Self {
            percent: 0.0,
            status: "Idle".into(),
            done: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGenParams {
    pub seed: u64,
    pub sea_level: u32,
    pub size: u32,
}

impl Default for WorldGenParams {
    fn default() -> Self {
        Self {
            seed: 12345,
            sea_level: 32,
            size: 256,
        }
    }
}

/// Shared between Bevy (writer) and Tauri commands (reader).
#[derive(Clone, Default)]
pub struct SharedGameState {
    pub hotbar: Arc<RwLock<[HotbarSlot; 9]>>,
    pub active_slot: Arc<RwLock<usize>>,
    pub combat: Arc<RwLock<CombatStateSnapshot>>,
    pub npcs: Arc<RwLock<HashMap<u64, NpcSnapshot>>>,
    pub generation: Arc<RwLock<GenerationProgress>>,
    pub world_params: Arc<RwLock<Option<WorldGenParams>>>,
    pub chunks_ready: Arc<RwLock<bool>>,
}
