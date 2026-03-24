use bevy::prelude::*;
use crate::components::NpcComponent;

const MOVE_SPEED: f32 = 3.0;
const WAYPOINT_WAIT: f32 = 2.0;
const ARRIVE_DIST: f32 = 0.3;

/// Moves each NPC along its waypoint list.
pub fn npc_routine(time: Res<Time>, mut npcs: Query<(&mut Transform, &mut NpcComponent)>) {
    let dt = time.delta_seconds();
    for (mut tf, mut npc) in &mut npcs {
        if npc.waypoints.is_empty() { continue; }

        let target = npc.waypoints[npc.waypoint_idx];
        let diff = target - tf.translation;
        let dist = diff.length();

        if dist < ARRIVE_DIST {
            npc.wait_timer += dt;
            if npc.wait_timer >= WAYPOINT_WAIT {
                npc.wait_timer = 0.0;
                npc.waypoint_idx = (npc.waypoint_idx + 1) % npc.waypoints.len();
            }
        } else {
            tf.translation += diff.normalize() * MOVE_SPEED * dt;
        }
    }
}
