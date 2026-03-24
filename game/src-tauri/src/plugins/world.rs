use bevy::prelude::*;
use crate::world::{
    chunk_manager::ChunkManager,
    terrain::TerrainGen,
    mesh::build_chunk_mesh,
};
use crate::components::Player;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
           .insert_resource(TerrainGenRes(TerrainGen::new(12345, 32)))
           .add_systems(Update, (load_chunks_around_player, spawn_dirty_chunks));
    }
}

#[derive(Resource)]
pub struct TerrainGenRes(pub TerrainGen);

fn load_chunks_around_player(
    player: Query<&Transform, With<Player>>,
    mut manager: ResMut<ChunkManager>,
    gen: Res<TerrainGenRes>,
) {
    let Ok(tf) = player.get_single() else { return };
    let (cx, cz) = ChunkManager::world_to_chunk(tf.translation.x, tf.translation.z);

    for (ncx, ncz) in ChunkManager::chunks_in_radius(cx, cz) {
        if manager.chunks.contains_key(&(ncx, ncz)) { continue; }
        let chunk = gen.0.generate_chunk(ncx, ncz);
        manager.insert(ncx, ncz, chunk);
    }
}

fn spawn_dirty_chunks(
    mut commands: Commands,
    mut manager: ResMut<ChunkManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dirty: Vec<(i32, i32)> = manager.dirty.drain(..).collect();
    for (cx, cz) in dirty {
        let Some(chunk) = manager.chunks.get(&(cx, cz)) else { continue };
        let mesh = build_chunk_mesh(chunk, cx, cz);
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.4, 0.7, 0.3)),
            ..default()
        });
    }
}
