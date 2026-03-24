use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::{Player, Inventory};
use crate::world::chunk_manager::ChunkManager;

/// Left-click: remove block at cursor. Right-click: place block from hotbar.
pub fn block_interaction(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut player: Query<&mut Inventory, With<Player>>,
    mut manager: ResMut<ChunkManager>,
) {
    let remove = buttons.just_pressed(MouseButton::Left);
    let place  = buttons.just_pressed(MouseButton::Right);
    if !remove && !place { return; }

    let Ok(win) = window.get_single() else { return };
    let Ok((cam, cam_tf)) = camera.get_single() else { return };
    let Some(cursor) = win.cursor_position() else { return };
    let Some(ray) = cam.viewport_to_world(cam_tf, cursor) else { return };

    // Intersect with y=0 ground plane
    if ray.direction.y.abs() < 1e-6 { return; }
    let t = -ray.origin.y / ray.direction.y;
    if t < 0.0 { return; }
    let hit = ray.origin + ray.direction * t;

    let wx = hit.x.floor() as i32;
    let wz = hit.z.floor() as i32;
    let (cx, cz) = (wx >> 4, wz >> 4);
    let (lx, lz) = (wx.rem_euclid(16) as usize, wz.rem_euclid(16) as usize);

    let Some(chunk) = manager.chunks.get_mut(&(cx, cz)) else { return };

    if remove {
        // Find top non-air block at this column
        for y in (0..64).rev() {
            if chunk.get(lx, y, lz) != 0 {
                let block_id = chunk.get(lx, y, lz);
                chunk.set(lx, y, lz, 0);
                manager.dirty.push((cx, cz));
                // Give block to player inventory
                if let Ok(mut inv) = player.get_single_mut() {
                    inv.add(block_id as u32, 1);
                }
                break;
            }
        }
    } else if place {
        let Ok(inv) = player.get_single() else { return };
        let (item_id, qty) = inv.hotbar_item();
        if item_id == 0 || qty == 0 { return; }
        // Place on top of column
        for y in (0..63).rev() {
            if chunk.get(lx, y, lz) != 0 {
                chunk.set(lx, y + 1, lz, item_id as u16);
                manager.dirty.push((cx, cz));
                break;
            }
        }
    }
}
