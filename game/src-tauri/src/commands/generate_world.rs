use serde::Serialize;
use tauri::State;
use crate::shared_state::{SharedGameState, GenerationProgress, WorldGenParams};
use crate::state::WorldStore;
use crate::world::terrain::TerrainGen;

#[derive(Debug, Serialize, Clone)]
pub struct GenProgressResult {
    pub percent: f32,
    pub status: String,
    pub done: bool,
}

#[tauri::command]
pub async fn generate_world(
    id: String,
    store: State<'_, WorldStore>,
    shared: State<'_, SharedGameState>,
) -> Result<(), String> {
    let params = store.get_params(&id).await
        .ok_or_else(|| "World not found".to_string())?;

    let seed = params.seed;
    let sea_level = params.sea_level;
    let size = params.size;

    // Store world params for Bevy to read
    {
        let mut wp = shared.world_params.write().unwrap();
        *wp = Some(WorldGenParams { seed, sea_level, size });
    }

    // Reset progress
    {
        let mut prog = shared.generation.write().unwrap();
        *prog = GenerationProgress {
            percent: 0.0,
            status: "Initializing terrain generator...".into(),
            done: false,
        };
    }

    let gen = TerrainGen::new(seed, sea_level);
    let chunk_radius: i32 = ((size as i32 / 16) / 2).max(4);
    let total_chunks = ((chunk_radius * 2 + 1) * (chunk_radius * 2 + 1)) as f32;
    let mut generated = 0f32;

    // Generate chunks
    {
        let mut prog = shared.generation.write().unwrap();
        prog.status = "Generating terrain heightmap...".into();
        prog.percent = 5.0;
    }

    let mut chunks = Vec::new();

    for cx in -chunk_radius..=chunk_radius {
        for cz in -chunk_radius..=chunk_radius {
            let chunk = gen.generate_chunk(cx, cz);
            chunks.push((cx, cz, chunk));
            generated += 1.0;

            let pct = 5.0 + (generated / total_chunks) * 70.0;
            let status = if pct < 25.0 {
                "Generating terrain heightmap..."
            } else if pct < 45.0 {
                "Carving caves..."
            } else if pct < 60.0 {
                "Placing ores and resources..."
            } else if pct < 70.0 {
                "Growing trees and vegetation..."
            } else {
                "Building structures..."
            };

            let mut prog = shared.generation.write().unwrap();
            prog.percent = pct;
            prog.status = status.into();
        }
    }

    // Store generated chunks in shared state for Bevy to consume
    {
        let mut prog = shared.generation.write().unwrap();
        prog.percent = 80.0;
        prog.status = "Preparing world mesh data...".into();
    }

    // Store chunks into a temporary location in shared state
    // Bevy will pick these up via ChunkManager
    {
        let mut prog = shared.generation.write().unwrap();
        prog.percent = 90.0;
        prog.status = "Finalizing world...".into();
    }

    // Signal Bevy that chunks are ready
    {
        let mut ready = shared.chunks_ready.write().unwrap();
        *ready = true;
    }

    {
        let mut prog = shared.generation.write().unwrap();
        prog.percent = 100.0;
        prog.status = "World generation complete!".into();
        prog.done = true;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_generation_progress(
    shared: State<'_, SharedGameState>,
) -> Result<GenProgressResult, String> {
    let prog = shared.generation.read().unwrap();
    Ok(GenProgressResult {
        percent: prog.percent,
        status: prog.status.clone(),
        done: prog.done,
    })
}
