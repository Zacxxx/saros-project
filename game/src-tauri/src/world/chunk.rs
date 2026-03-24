/// 16×16×64 chunk stored as Structure of Arrays.
/// Index: x + z*16 + y*256  (x,z in 0..16, y in 0..64)
pub const CHUNK_W: usize = 16;
pub const CHUNK_D: usize = 16;
pub const CHUNK_H: usize = 64;
pub const CHUNK_VOL: usize = CHUNK_W * CHUNK_D * CHUNK_H;

#[derive(Clone)]
pub struct Chunk {
    pub block_ids: Vec<u16>,   // block type per voxel
    pub light:     Vec<u8>,    // light level per voxel
}

impl Chunk {
    pub fn empty() -> Self {
        Self {
            block_ids: vec![0; CHUNK_VOL],
            light:     vec![0; CHUNK_VOL],
        }
    }

    #[inline]
    pub fn idx(x: usize, y: usize, z: usize) -> usize {
        x + z * CHUNK_W + y * CHUNK_W * CHUNK_D
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> u16 {
        self.block_ids[Self::idx(x, y, z)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, id: u16) {
        let i = Self::idx(x, y, z);
        self.block_ids[i] = id;
    }
}
