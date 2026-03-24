use super::chunk::{CHUNK_D, CHUNK_H, CHUNK_W, Chunk};
use noise::{Fbm, NoiseFn, Perlin};

// Block type IDs
pub const BLOCK_AIR: u16 = 0;
pub const BLOCK_STONE: u16 = 1;
pub const BLOCK_DIRT: u16 = 2;
pub const BLOCK_GRASS: u16 = 3;
pub const BLOCK_WATER: u16 = 4;
pub const BLOCK_SAND: u16 = 5;
pub const BLOCK_LOG: u16 = 6;
pub const BLOCK_LEAF: u16 = 7;
pub const BLOCK_OAK_PLANKS: u16 = 8;
pub const BLOCK_COBBLESTONE: u16 = 9;
pub const BLOCK_IRON_ORE: u16 = 10;
pub const BLOCK_COAL_ORE: u16 = 11;
pub const BLOCK_GOLD_ORE: u16 = 12;
pub const BLOCK_DIAMOND_ORE: u16 = 13;
pub const BLOCK_GRAVEL: u16 = 14;
pub const BLOCK_CLAY: u16 = 15;
pub const BLOCK_SNOW: u16 = 16;
pub const BLOCK_ICE: u16 = 17;
pub const BLOCK_OBSIDIAN: u16 = 18;
pub const BLOCK_BEDROCK: u16 = 19;
pub const BLOCK_MOSSY_COBBLESTONE: u16 = 20;
pub const BLOCK_BRICKS: u16 = 21;
pub const BLOCK_BOOKSHELF: u16 = 22;
pub const BLOCK_GLOWSTONE: u16 = 23;

pub const BLOCK_COUNT: u16 = 24;

/// Names for each block ID (indexed by block ID).
pub const BLOCK_NAMES: [&str; 24] = [
    "Air",
    "Stone",
    "Dirt",
    "Grass",
    "Water",
    "Sand",
    "Oak Log",
    "Oak Leaves",
    "Oak Planks",
    "Cobblestone",
    "Iron Ore",
    "Coal Ore",
    "Gold Ore",
    "Diamond Ore",
    "Gravel",
    "Clay",
    "Snow",
    "Ice",
    "Obsidian",
    "Bedrock",
    "Mossy Cobblestone",
    "Bricks",
    "Bookshelf",
    "Glowstone",
];

/// Texture file names for each block (indexed by block ID).
pub const BLOCK_TEXTURES: [&str; 24] = [
    "",
    "stone.png",
    "dirt.png",
    "grass_block_top.png",
    "water_still.png",
    "sand.png",
    "oak_log.png",
    "oak_leaves.png",
    "oak_planks.png",
    "cobblestone.png",
    "iron_ore.png",
    "coal_ore.png",
    "gold_ore.png",
    "diamond_ore.png",
    "gravel.png",
    "clay.png",
    "snow.png",
    "ice.png",
    "obsidian.png",
    "bedrock.png",
    "mossy_cobblestone.png",
    "bricks.png",
    "bookshelf.png",
    "glowstone.png",
];

/// Whether a block is solid (opaque, blocks visibility).
pub const BLOCK_SOLID: [bool; 24] = [
    false, true, true, true, false, true, true, false, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true,
];

/// Whether a block is transparent (for face culling).
pub const BLOCK_TRANSPARENT: [bool; 24] = [
    true, false, false, false, true, false, false, true, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false,
];

pub struct TerrainGen {
    fbm: Fbm<Perlin>,
    cave: Perlin,
    tree: Perlin,
    ore: Perlin,
    biome: Perlin,
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
            ore: Perlin::new(seed as u32 + 3),
            biome: Perlin::new(seed as u32 + 4),
            sea_level,
        }
    }

    pub fn generate_chunk(&self, cx: i32, cz: i32) -> Chunk {
        let mut chunk = Chunk::empty();
        let sea = self.sea_level as usize;

        // --- Biome map ---
        let mut biome_vals = [[0.0f64; CHUNK_D]; CHUNK_W];
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.003;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.003;
                biome_vals[x][z] = self.biome.get([wx, wz]);
            }
        }

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
                let bv = biome_vals[x][z];
                let is_cold = bv > 0.5;
                let is_beach = height <= sea + 2 && height > sea;

                for y in 0..CHUNK_H {
                    let id = if y == 0 {
                        BLOCK_BEDROCK
                    } else if y < height.saturating_sub(3) {
                        BLOCK_STONE
                    } else if y < height {
                        BLOCK_DIRT
                    } else if y == height {
                        if height <= sea {
                            BLOCK_SAND
                        } else if is_cold && height > sea + 10 {
                            BLOCK_SNOW
                        } else if is_beach {
                            BLOCK_SAND
                        } else {
                            BLOCK_GRASS
                        }
                    } else if y <= sea {
                        if is_cold && y == sea {
                            BLOCK_ICE
                        } else {
                            BLOCK_WATER
                        }
                    } else {
                        BLOCK_AIR
                    };
                    chunk.set(x, y, z, id);
                }
            }
        }

        // --- Ore placement pass ---
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.08;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.08;
                for y in 1..sea.saturating_sub(4) {
                    if chunk.get(x, y, z) != BLOCK_STONE {
                        continue;
                    }
                    let wy = y as f64 * 0.08;
                    let n = self.ore.get([wx, wy, wz]);

                    let ore_block = if y < 12 && n > 0.82 {
                        BLOCK_DIAMOND_ORE
                    } else if y < 20 && n > 0.78 {
                        BLOCK_GOLD_ORE
                    } else if y < 40 && n > 0.72 {
                        BLOCK_IRON_ORE
                    } else if n > 0.68 {
                        BLOCK_COAL_ORE
                    } else if n < -0.75 {
                        BLOCK_GRAVEL
                    } else {
                        continue;
                    };
                    chunk.set(x, y, z, ore_block);
                }
            }
        }

        // --- Cave carving pass (3D Perlin) ---
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.05;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.05;
                for y in 1..(sea.saturating_sub(2)) {
                    if chunk.get(x, y, z) == BLOCK_BEDROCK {
                        continue;
                    }
                    let wy = y as f64 * 0.05;
                    let n = self.cave.get([wx, wy, wz]);
                    if n > 0.55 {
                        chunk.set(x, y, z, BLOCK_AIR);
                    }
                }
            }
        }

        // --- Tree placement pass ---
        for z in 2..(CHUNK_D - 2) {
            for x in 2..(CHUNK_W - 2) {
                let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.3;
                let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.3;
                let height = heights[x][z];
                if height <= sea {
                    continue;
                }
                let bv = biome_vals[x][z];
                if bv > 0.5 {
                    continue;
                } // no trees in cold biome
                let t = self.tree.get([wx, wz]);
                if t > 0.7 {
                    self.place_tree(&mut chunk, x, height + 1, z);
                }
            }
        }

        // --- Clay patches near water ---
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let height = heights[x][z];
                if height == sea || height == sea - 1 {
                    let wx = (cx * CHUNK_W as i32 + x as i32) as f64 * 0.15;
                    let wz = (cz * CHUNK_D as i32 + z as i32) as f64 * 0.15;
                    let n = self.ore.get([wx, 100.0, wz]);
                    if n > 0.6 && chunk.get(x, height, z) == BLOCK_SAND {
                        chunk.set(x, height, z, BLOCK_CLAY);
                    }
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
            if y >= CHUNK_H {
                return;
            }
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
