use super::chunk::{CHUNK_D, CHUNK_H, CHUNK_W, Chunk};
use super::terrain::*;
use serde::{Deserialize, Serialize};

/// A single block placement within a structure (relative coordinates).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureBlock {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block_name: String,
}

/// A structure template that can be placed in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub name: String,
    pub size: [u32; 3], // width, height, depth
    pub blocks: Vec<StructureBlock>,
}

/// Registry of all available structure templates.
pub struct StructureRegistry {
    pub structures: Vec<Structure>,
}

impl StructureRegistry {
    /// Load built-in structure templates.
    pub fn new() -> Self {
        Self {
            structures: vec![
                Self::oak_tree(),
                Self::tall_oak_tree(),
                Self::birch_tree(),
                Self::boulder(),
            ],
        }
    }

    pub fn get(&self, name: &str) -> Option<&Structure> {
        self.structures.iter().find(|s| s.name == name)
    }

    fn oak_tree() -> Structure {
        let mut blocks = Vec::new();
        // Trunk: 4 blocks tall
        for y in 0..4 {
            blocks.push(StructureBlock {
                x: 0,
                y,
                z: 0,
                block_name: "oak_log".into(),
            });
        }
        // Leaves: 3×3×2 cap at top
        for dy in 4..6 {
            for dx in -1..=1i32 {
                for dz in -1..=1i32 {
                    if dx == 0 && dz == 0 && dy == 4 {
                        continue;
                    } // trunk continues
                    blocks.push(StructureBlock {
                        x: dx,
                        y: dy,
                        z: dz,
                        block_name: "oak_leaves".into(),
                    });
                }
            }
        }
        // Top trunk
        blocks.push(StructureBlock {
            x: 0,
            y: 4,
            z: 0,
            block_name: "oak_log".into(),
        });
        Structure {
            name: "oak_tree".into(),
            size: [3, 6, 3],
            blocks,
        }
    }

    fn tall_oak_tree() -> Structure {
        let mut blocks = Vec::new();
        // Trunk: 6 blocks tall
        for y in 0..6 {
            blocks.push(StructureBlock {
                x: 0,
                y,
                z: 0,
                block_name: "oak_log".into(),
            });
        }
        // Leaves: 5×5 at y=4, 3×3 at y=5,6, 1×1 at y=7
        for dx in -2..=2i32 {
            for dz in -2..=2i32 {
                if dx.abs() == 2 && dz.abs() == 2 {
                    continue;
                }
                blocks.push(StructureBlock {
                    x: dx,
                    y: 4,
                    z: dz,
                    block_name: "oak_leaves".into(),
                });
            }
        }
        for dy in 5..=6 {
            for dx in -1..=1i32 {
                for dz in -1..=1i32 {
                    blocks.push(StructureBlock {
                        x: dx,
                        y: dy,
                        z: dz,
                        block_name: "oak_leaves".into(),
                    });
                }
            }
        }
        blocks.push(StructureBlock {
            x: 0,
            y: 7,
            z: 0,
            block_name: "oak_leaves".into(),
        });
        Structure {
            name: "tall_oak_tree".into(),
            size: [5, 8, 5],
            blocks,
        }
    }

    fn birch_tree() -> Structure {
        let mut blocks = Vec::new();
        for y in 0..5 {
            blocks.push(StructureBlock {
                x: 0,
                y,
                z: 0,
                block_name: "oak_log".into(),
            });
        }
        for dy in 3..6 {
            for dx in -1..=1i32 {
                for dz in -1..=1i32 {
                    if dx == 0 && dz == 0 {
                        continue;
                    }
                    blocks.push(StructureBlock {
                        x: dx,
                        y: dy,
                        z: dz,
                        block_name: "oak_leaves".into(),
                    });
                }
            }
        }
        blocks.push(StructureBlock {
            x: 0,
            y: 5,
            z: 0,
            block_name: "oak_leaves".into(),
        });
        Structure {
            name: "birch_tree".into(),
            size: [3, 6, 3],
            blocks,
        }
    }

    fn boulder() -> Structure {
        let mut blocks = Vec::new();
        // 3×2×3 cobblestone with mossy cobblestone accents
        for dy in 0..2 {
            for dx in -1..=1i32 {
                for dz in -1..=1i32 {
                    if dy == 1 && (dx.abs() + dz.abs() > 1) {
                        continue;
                    }
                    let name = if (dx + dz + dy) % 3 == 0 {
                        "mossy_cobblestone"
                    } else {
                        "cobblestone"
                    };
                    blocks.push(StructureBlock {
                        x: dx,
                        y: dy,
                        z: dz,
                        block_name: name.into(),
                    });
                }
            }
        }
        Structure {
            name: "boulder".into(),
            size: [3, 2, 3],
            blocks,
        }
    }
}

/// Resolve a block name to its block ID.
pub fn block_name_to_id(name: &str) -> u16 {
    match name {
        "oak_log" => BLOCK_LOG,
        "oak_leaves" => BLOCK_LEAF,
        "oak_planks" => BLOCK_OAK_PLANKS,
        "cobblestone" => BLOCK_COBBLESTONE,
        "mossy_cobblestone" => BLOCK_MOSSY_COBBLESTONE,
        "stone" => BLOCK_STONE,
        "bricks" => BLOCK_BRICKS,
        "bookshelf" => BLOCK_BOOKSHELF,
        "glowstone" => BLOCK_GLOWSTONE,
        _ => BLOCK_STONE,
    }
}

/// Place a structure in a chunk at the given position.
pub fn place_structure(
    chunk: &mut Chunk,
    structure: &Structure,
    base_x: usize,
    base_y: usize,
    base_z: usize,
) {
    for sb in &structure.blocks {
        let x = base_x as i32 + sb.x;
        let y = base_y as i32 + sb.y;
        let z = base_z as i32 + sb.z;

        if x < 0
            || x >= CHUNK_W as i32
            || y < 0
            || y >= CHUNK_H as i32
            || z < 0
            || z >= CHUNK_D as i32
        {
            continue;
        }

        let block_id = block_name_to_id(&sb.block_name);
        let current = chunk.get(x as usize, y as usize, z as usize);
        // Only place if the target is air or we're replacing air-like blocks
        if current == BLOCK_AIR || current == BLOCK_LEAF {
            chunk.set(x as usize, y as usize, z as usize, block_id);
        }
    }
}
