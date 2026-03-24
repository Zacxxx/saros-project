use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobType {
    Passive,
    Hostile,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobBehavior {
    Idle,
    Wander,
    Chase,
    Flee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntry {
    pub item_id: u32,
    pub quantity_min: u32,
    pub quantity_max: u32,
    pub chance: f32,
}

#[derive(Component)]
pub struct MobComponent {
    pub id: u64,
    pub name: String,
    pub mob_type: MobType,
    pub can_talk: bool,
    pub ai_thoughts: String,
    pub thought_timer: f32,
    pub loot_table: Vec<LootEntry>,
    pub hp: i32,
    pub max_hp: i32,
    pub behavior: MobBehavior,
    pub wander_timer: f32,
    pub wander_target: Option<Vec3>,
}

impl MobComponent {
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
