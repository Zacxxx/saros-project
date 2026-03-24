use bevy::prelude::*;
use crate::components::NpcComponent;
use crate::systems::npc_routine::npc_routine;
use crate::systems::npc_thought::npc_thought_tick;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npcs)
           .add_systems(Update, (npc_routine, npc_thought_tick));
    }
}

fn spawn_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let npcs = vec![
        ("Aldric", vec!["wise", "friendly"], vec![Vec3::new(5., 1., 5.), Vec3::new(10., 1., 5.), Vec3::new(10., 1., 10.)]),
        ("Mira",  vec!["curious", "cautious"], vec![Vec3::new(-5., 1., 3.), Vec3::new(-8., 1., 8.)]),
    ];

    for (i, (name, traits, waypoints)) in npcs.into_iter().enumerate() {
        let start = waypoints[0];
        commands.spawn((
            NpcComponent {
                id: i as u64 + 1,
                name: name.into(),
                traits: traits.into_iter().map(String::from).collect(),
                waypoints,
                waypoint_idx: 0,
                wait_timer: 0.0,
                memory: vec![],
                current_thought: String::new(),
                thought_timer: 0.0,
            },
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.7, 1.8, 0.7)),
                material: materials.add(Color::rgb(0.8, 0.5, 0.2)),
                transform: Transform::from_translation(start),
                ..default()
            },
        ));
    }
}
