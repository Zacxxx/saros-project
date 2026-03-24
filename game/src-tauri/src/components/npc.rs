use bevy::prelude::*;

#[derive(Component)]
pub struct NpcComponent {
    pub id: u64,
    pub name: String,
    pub traits: Vec<String>,
    pub waypoints: Vec<Vec3>,
    pub waypoint_idx: usize,
    pub wait_timer: f32,
    pub memory: Vec<(String, String)>, // (role, content)
    pub current_thought: String,
    pub thought_timer: f32,
}
