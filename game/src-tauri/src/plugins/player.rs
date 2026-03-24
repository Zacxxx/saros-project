use bevy::prelude::*;
use crate::components::{Player, CameraTarget, GridPos, MovePath, Inventory, CombatStats};
use crate::systems::mouse_input::handle_mouse_click;
use crate::systems::move_along_path::move_along_path;
use crate::systems::block_interaction::block_interaction;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
           .add_systems(Update, (handle_mouse_click, move_along_path, block_interaction));
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        CameraTarget,
        GridPos(0, 0),
        MovePath::default(),
        Inventory::default(),
        CombatStats::default(),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.8, 1.6, 0.8)),
            material: materials.add(Color::rgb(0.4, 0.6, 1.0)),
            transform: Transform::from_xyz(0.5, 0.8, 0.5),
            ..default()
        },
    ));
}
