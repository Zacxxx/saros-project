use bevy::prelude::*;
use crate::components::{Player, GridPos, MovePath};
use crate::world::chunk_manager::ChunkManager;

const STEP_INTERVAL: f32 = 0.15; // seconds per grid step

/// Advances the player one grid step along their MovePath.
pub fn move_along_path(
    time: Res<Time>,
    manager: Res<ChunkManager>,
    mut player: Query<(&mut Transform, &mut GridPos, &mut MovePath), With<Player>>,
) {
    let Ok((mut tf, mut pos, mut path)) = player.get_single_mut() else { return };
    if path.steps.is_empty() { return; }

    path.timer += time.delta_seconds();
    if path.timer < STEP_INTERVAL { return; }
    path.timer = 0.0;

    let Some((nx, nz)) = path.steps.pop_front() else { return };
    pos.0 = nx;
    pos.1 = nz;

    // Find surface y at new position
    let y = surface_y(&manager, nx, nz);
    tf.translation = Vec3::new(nx as f32 + 0.5, y as f32 + 0.8, nz as f32 + 0.5);
}

fn surface_y(manager: &ChunkManager, x: i32, z: i32) -> usize {
    let (cx, cz) = (x >> 4, z >> 4);
    let (lx, lz) = (x.rem_euclid(16) as usize, z.rem_euclid(16) as usize);
    let Some(chunk) = manager.get(cx, cz) else { return 0 };
    for y in (0..64).rev() {
        if chunk.get(lx, y, lz) != 0 { return y; }
    }
    0
}
