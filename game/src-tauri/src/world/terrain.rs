use noise::{NoiseFn, Perlin, Fbm};
use super::chunk::{Chunk, CHUNK_W, CHUNK_D, CHUNK_H};

pub const BLOCK_AIR:   u16 = 0;
pub const BLOCK_STONE: u16 = 1;
pub const BLOCK_DIRT:  u16 = 2;
pub const BLOCK_GRASS: u16 = 3;
pub const BLOCK_WATER: u16 = 4;
pub const BLOCK_SAND:  u16 = 5;
pub const BLOCK_LOG:   u16 = 6;
pub const BLOCK_LEAF:  u16 = 7;

pub struct TerrainGen {
    fbm:   Fbm<Perlin>,
    cave:  Perlin,
    tree:  Perlin,
    sea_level: u32,
}

impl TerrainGen {
    pub fn new(seed: u64, sea_level: u32) -> Self {
        let mut fbm = Fbm::<Perlin>::new(seed as u32);
        fbm.octaves = 5;
        fbm.frequency = 0.008;
        fbm.lacunarity = 2.0;
        fbm.persistence = 0.5;
        Self {
            fbm,
            cave: Perlin::new(seed as u32 + 1),
            tree: Perlin::new(seed as u32 + 2),
            sea_level,
        }
    }

    pub fn generate_chunk(&self, cx: i32, cz: i32) -> Chunk {
        let mut chunk = Chunk::empty();
        let sea = self.sea_level as usize;

        // --- Heightmap pass ---
        let mut heights = [[0usize; CHUNK_D]; CHUNK_W];
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64;
                let n = self.fbm.get([wx, wz]);
                let h = ((n + 1.0) * 0.5 * (CHUNK_H as f64 * 0.6)) as usize;
                heights[x][z] = h.clamp(1, CHUNK_H - 1);
            }
        }

        // --- Block fill pass ---
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let height = heights[x][z];
                for y in 0..CHUNK_H {
                    let id = if y == 0 {
                        BLOCK_STONE
                    } else if y < height.saturating_sub(3) {
                        BLOCK_STONE
                    } else if y < height {
                        BLOCK_DIRT
                    } else if y == height {
                        if height <= sea { BLOCK_SAND } else { BLOCK_GRASS }
                    } else if y <= sea {
                        BLOCK_WATER
                    } else {
                        BLOCK_AIR
                    };
                    chunk.set(x, y, z, id);
                }
            }
        }

        // --- Cave carving pass (3D Perlin) ---
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.05;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.05;
                for y in 1..(sea.saturating_sub(2)) {
                    let wy = y as f64 * 0.05;
                    let n = self.cave.get([wx, wy, wz]);
                    if n > 0.55 {
                        chunk.set(x, y, z, BLOCK_AIR);
                    }
                }
            }
        }

        // --- Tree placement pass ---
        for z in 1..(CHUNK_D - 1) {
            for x in 1..(CHUNK_W - 1) {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.3;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.3;
                let height = heights[x][z];
                if height <= sea { continue; }
                let t = self.tree.get([wx, wz]);
                if t > 0.7 {
                    self.place_tree(&mut chunk, x, height + 1, z);
                }
            }
        }

        chunk
    }

    fn place_tree(&self, chunk: &mut Chunk, x: usize, base_y: usize, z: usize) {
        let trunk_h = 4;
        // Trunk
        for dy in 0..trunk_h {
            let y = base_y + dy;
            if y >= CHUNK_H { return; }
            chunk.set(x, y, z, BLOCK_LOG);
        }
        // Leaves (3×3×2 cap)
        let top = base_y + trunk_h;
        for dy in 0..2usize {
            for dz in 0usize..3 {
                for dx in 0usize..3 {
                    let lx = x.wrapping_add(dx).wrapping_sub(1);
                    let ly = top + dy;
                    let lz = z.wrapping_add(dz).wrapping_sub(1);
                    if lx < CHUNK_W && ly < CHUNK_H && lz < CHUNK_D {
                        if chunk.get(lx, ly, lz) == BLOCK_AIR {
                            chunk.set(lx, ly, lz, BLOCK_LEAF);
                        }
                    }
                }
            }
        }
    }
}
