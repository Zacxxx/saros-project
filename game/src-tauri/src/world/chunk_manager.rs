use std::collections::HashMap;
use bevy::prelude::*;
use super::chunk::Chunk;

pub const LOAD_RADIUS: i32 = 4;

#[derive(Resource, Default)]
pub struct ChunkManager {
    pub chunks: HashMap<(i32, i32), Chunk>,
    pub dirty:  Vec<(i32, i32)>,
}

impl ChunkManager {
    pub fn get(&self, cx: i32, cz: i32) -> Option<&Chunk> {
        self.chunks.get(&(cx, cz))
    }

    pub fn insert(&mut self, cx: i32, cz: i32, chunk: Chunk) {
        self.chunks.insert((cx, cz), chunk);
        self.dirty.push((cx, cz));
    }

    pub fn world_to_chunk(x: f32, z: f32) -> (i32, i32) {
        (x.floor() as i32 >> 4, z.floor() as i32 >> 4)
    }

    /// Returns chunk coords that should be loaded around a center.
    pub fn chunks_in_radius(cx: i32, cz: i32) -> impl Iterator<Item = (i32, i32)> {
        let r = LOAD_RADIUS;
        (-r..=r).flat_map(move |dx| (-r..=r).map(move |dz| (cx + dx, cz + dz)))
    }
}
