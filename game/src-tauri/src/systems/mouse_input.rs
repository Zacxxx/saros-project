use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::{Player, GridPos, MovePath};
use crate::world::chunk_manager::ChunkManager;
use super::pathfinding::astar;

/// On left-click: raycast to ground plane → A* → populate MovePath.
pub fn handle_mouse_click(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut player: Query<(&GridPos, &mut MovePath), With<Player>>,
    manager: Res<ChunkManager>,
) {
    if !buttons.just_pressed(MouseButton::Left) { return; }

    let Ok(win) = window.get_single() else { return };
    let Ok((cam, cam_tf)) = camera.get_single() else { return };
    let Some(cursor) = win.cursor_position() else { return };
    let Ok((start_pos, mut path)) = player.get_single_mut() else { return };

    // Unproject cursor to world ray
    let Some(ray) = cam.viewport_to_world(cam_tf, cursor) else { return };

    // Intersect ray with y = 0 plane (approximate ground)
    let t = -ray.origin.y / ray.direction.y;
    if t < 0.0 { return; }
    let hit = ray.origin + ray.direction * t;
    let goal = (hit.x.floor() as i32, hit.z.floor() as i32);

    let is_walkable = |x: i32, z: i32| {
        let (cx, cz) = (x >> 4, z >> 4);
        let (lx, lz) = (x.rem_euclid(16) as usize, z.rem_euclid(16) as usize);
        let Some(chunk) = manager.get(cx, cz) else { return false };
        // Walkable: some solid block below, air at walk height
        (1..64).any(|y| chunk.get(lx, y, lz) != 0)
    };

    if let Some(new_path) = astar((start_pos.0, start_pos.1), goal, is_walkable) {
        path.steps = new_path.into_iter().skip(1).collect();
    }
}
