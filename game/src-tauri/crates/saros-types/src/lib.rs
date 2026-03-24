use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldParams {
    pub name: String,
    pub seed: u64,
    pub size: u32,
    pub sea_level: u32,
    pub cave_density: f32,
    pub structure_frequency: f32,
    pub biome_weights: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldRecord {
    pub id: String,
    pub name: String,
    pub seed: u64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub name: String,
    pub texture_id: String,
    pub is_solid: bool,
    pub is_transparent: bool,
    pub hardness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub texture_id: String,
    pub stack_size: u32,
    pub item_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item_id: u64,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u64,
    pub output: ItemStack,
    pub inputs: Vec<ItemStack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub id: u64,
    pub world_id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub traits: Vec<String>,
    pub current_thought: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: u64,
    pub world_id: u64,
    pub title: String,
    pub description: String,
    pub giver_npc_id: u64,
    pub objectives: Vec<QuestObjective>,
    pub rewards: Vec<ItemStack>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum QuestObjective {
    CollectItems { item_id: u64, quantity: u32 },
    DefeatEnemies { enemy_type: String, count: u32 },
    ReachLocation { x: f32, y: f32, z: f32 },
    TalkToNpc { npc_id: u64 },
}
