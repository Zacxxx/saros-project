use bevy::prelude::*;
use crate::components::{Player, CombatStats, GridPos};
use crate::plugins::combat::TurnState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnState>()
           .add_systems(Update, end_turn_on_key);
    }
}

/// Press Space to end player turn.
fn end_turn_on_key(
    keys: Res<ButtonInput<KeyCode>>,
    mut turn: ResMut<TurnState>,
    mut player: Query<&mut CombatStats, With<Player>>,
) {
    if !keys.just_pressed(KeyCode::Space) { return; }
    if let Ok(mut stats) = player.get_single_mut() {
        // Restore AP/MP at start of new turn
        stats.ap = stats.max_ap;
        stats.mp = stats.max_mp;
    }
    turn.round += 1;
}
