use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub ap: u32,     // action points
    pub max_ap: u32,
    pub mp: u32,     // movement points
    pub max_mp: u32,
    pub initiative: i32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self { hp: 100, max_hp: 100, ap: 6, max_ap: 6, mp: 3, max_mp: 3, initiative: 10 }
    }
}
