use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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

/// Shared between Bevy (writer) and Tauri commands (reader).
#[derive(Clone, Default)]
pub struct SharedGameState {
    pub hotbar:      Arc<RwLock<[HotbarSlot; 9]>>,
    pub active_slot: Arc<RwLock<usize>>,
    pub combat:      Arc<RwLock<CombatStateSnapshot>>,
    pub npcs:        Arc<RwLock<HashMap<u64, NpcSnapshot>>>,
}
