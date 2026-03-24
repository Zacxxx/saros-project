use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use crate::world::{
    chunk_manager::ChunkManager,
    terrain::TerrainGen,
    mesh::build_chunk_mesh,
};
use crate::components::Player;
use crate::systems::sync_inventory::GameStateBridge;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkManager::default())
           .add_systems(Startup, setup_world)
           .add_systems(Update, (load_chunks_around_player, spawn_dirty_chunks));
    }
}

#[derive(Resource)]
pub struct TerrainGenRes(pub TerrainGen);

#[derive(Resource)]
pub struct BlockAtlasMaterial(pub Handle<StandardMaterial>);

/// Build a simple color atlas texture at startup (6 cols × 4 rows = 24 block types).
fn setup_world(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    bridge: Option<Res<GameStateBridge>>,
) {
    // Read world params from shared state, or use defaults
    let (seed, sea_level) = if let Some(bridge) = bridge {
        let wp = bridge.0.world_params.read().unwrap();
        if let Some(ref params) = *wp {
            (params.seed, params.sea_level)
        } else {
            (12345u64, 32u32)
        }
    } else {
        (12345u64, 32u32)
    };

    commands.insert_resource(TerrainGenRes(TerrainGen::new(seed, sea_level)));

    // Create a simple color-based texture atlas (6×4 tiles, each tile 16×16 pixels)
    let tile_size = 16u32;
    let cols = 6u32;
    let rows = 4u32;
    let width = cols * tile_size;
    let height = rows * tile_size;

    // Block colors (RGBA) indexed by block ID 1..24
    let block_colors: [[u8; 4]; 24] = [
        [0, 0, 0, 0],          // 0: Air (unused)
        [128, 128, 128, 255],   // 1: Stone
        [139, 90, 43, 255],     // 2: Dirt
        [86, 152, 40, 255],     // 3: Grass
        [30, 80, 180, 160],     // 4: Water
        [219, 199, 145, 255],   // 5: Sand
        [107, 83, 47, 255],     // 6: Oak Log
        [50, 120, 30, 200],     // 7: Oak Leaves
        [183, 148, 95, 255],    // 8: Oak Planks
        [110, 110, 110, 255],   // 9: Cobblestone
        [160, 140, 120, 255],   // 10: Iron Ore
        [60, 60, 60, 255],      // 11: Coal Ore
        [200, 185, 80, 255],    // 12: Gold Ore
        [80, 220, 220, 255],    // 13: Diamond Ore
        [140, 130, 125, 255],   // 14: Gravel
        [160, 165, 170, 255],   // 15: Clay
        [240, 245, 250, 255],   // 16: Snow
        [160, 200, 240, 200],   // 17: Ice
        [20, 15, 30, 255],      // 18: Obsidian
        [50, 50, 50, 255],      // 19: Bedrock
        [100, 130, 90, 255],    // 20: Mossy Cobblestone
        [155, 85, 65, 255],     // 21: Bricks
        [115, 95, 60, 255],     // 22: Bookshelf
        [200, 180, 100, 255],   // 23: Glowstone
    ];

    let mut data = vec![0u8; (width * height * 4) as usize];

    for block_id in 1u32..24u32 {
        let col = (block_id - 1) % cols;
        let row = (block_id - 1) / cols;
        let color = block_colors[block_id as usize];

        for ty in 0..tile_size {
            for tx in 0..tile_size {
                let px = col * tile_size + tx;
                let py = row * tile_size + ty;
                let offset = ((py * width + px) * 4) as usize;
                data[offset]   = color[0];
                data[offset+1] = color[1];
                data[offset+2] = color[2];
                data[offset+3] = color[3];
            }
        }
    }

    let image = Image::new(
        bevy::render::render_resource::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    let image_handle = images.add(image);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        perceptual_roughness: 0.9,
        metallic: 0.0,
        reflectance: 0.1,
        ..default()
    });

    commands.insert_resource(BlockAtlasMaterial(material));
}

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
    atlas_mat: Option<Res<BlockAtlasMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dirty: Vec<(i32, i32)> = manager.dirty.drain(..).collect();
    if dirty.is_empty() { return; }

    let material = if let Some(atlas) = atlas_mat {
        atlas.0.clone()
    } else {
        // Fallback: flat green
        materials.add(Color::rgb(0.4, 0.7, 0.3))
    };

    for (cx, cz) in dirty {
        let Some(chunk) = manager.chunks.get(&(cx, cz)) else { continue };
        let mesh = build_chunk_mesh(chunk, cx, cz);
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: material.clone(),
            ..default()
        });
    }
}
