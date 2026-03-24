use super::chunk::{CHUNK_D, CHUNK_H, CHUNK_W, Chunk};
use super::terrain::{BLOCK_COUNT, BLOCK_TRANSPARENT};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

/// Atlas layout: 6 columns × 4 rows = 24 tiles
const ATLAS_COLS: f32 = 6.0;
const ATLAS_ROWS: f32 = 4.0;

/// Returns (u_min, v_min, u_max, v_max) for a block ID in the texture atlas.
fn block_uv_rect(block_id: u16) -> (f32, f32, f32, f32) {
    if block_id == 0 || block_id >= BLOCK_COUNT {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let idx = (block_id - 1) as f32; // 0-indexed
    let col = idx % ATLAS_COLS;
    let row = (idx / ATLAS_COLS).floor();
    let u0 = col / ATLAS_COLS;
    let v0 = row / ATLAS_ROWS;
    let u1 = (col + 1.0) / ATLAS_COLS;
    let v1 = (row + 1.0) / ATLAS_ROWS;
    (u0, v0, u1, v1)
}

/// Generates a mesh from visible (non-air) block faces with UV coordinates.
pub fn build_chunk_mesh(chunk: &Chunk, cx: i32, cz: i32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let ox = (cx * CHUNK_W as i32) as f32;
    let oz = (cz * CHUNK_D as i32) as f32;

    for y in 0..CHUNK_H {
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                let block_id = chunk.get(x, y, z);
                if block_id == 0 {
                    continue;
                }

                let fx = ox + x as f32;
                let fy = y as f32;
                let fz = oz + z as f32;

                let (u0, v0, u1, v1) = block_uv_rect(block_id);

                // Face UVs (same for all faces of a block)
                let face_uvs: [[f32; 2]; 4] = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];

                // Emit each face only if the neighbour is air or transparent
                let faces: &[([f32; 3], [[f32; 3]; 4])] = &[
                    // +Y top
                    (
                        [0., 1., 0.],
                        [
                            [fx, fy + 1., fz],
                            [fx + 1., fy + 1., fz],
                            [fx + 1., fy + 1., fz + 1.],
                            [fx, fy + 1., fz + 1.],
                        ],
                    ),
                    // -Y bottom
                    (
                        [0., -1., 0.],
                        [
                            [fx, fy, fz + 1.],
                            [fx + 1., fy, fz + 1.],
                            [fx + 1., fy, fz],
                            [fx, fy, fz],
                        ],
                    ),
                    // +X right
                    (
                        [1., 0., 0.],
                        [
                            [fx + 1., fy, fz],
                            [fx + 1., fy + 1., fz],
                            [fx + 1., fy + 1., fz + 1.],
                            [fx + 1., fy, fz + 1.],
                        ],
                    ),
                    // -X left
                    (
                        [-1., 0., 0.],
                        [
                            [fx, fy, fz + 1.],
                            [fx, fy + 1., fz + 1.],
                            [fx, fy + 1., fz],
                            [fx, fy, fz],
                        ],
                    ),
                    // +Z front
                    (
                        [0., 0., 1.],
                        [
                            [fx, fy, fz + 1.],
                            [fx + 1., fy, fz + 1.],
                            [fx + 1., fy + 1., fz + 1.],
                            [fx, fy + 1., fz + 1.],
                        ],
                    ),
                    // -Z back
                    (
                        [0., 0., -1.],
                        [
                            [fx + 1., fy, fz],
                            [fx, fy, fz],
                            [fx, fy + 1., fz],
                            [fx + 1., fy + 1., fz],
                        ],
                    ),
                ];

                let neighbors: [(i32, i32, i32); 6] = [
                    (x as i32, y as i32 + 1, z as i32),
                    (x as i32, y as i32 - 1, z as i32),
                    (x as i32 + 1, y as i32, z as i32),
                    (x as i32 - 1, y as i32, z as i32),
                    (x as i32, y as i32, z as i32 + 1),
                    (x as i32, y as i32, z as i32 - 1),
                ];

                for (i, (normal, verts)) in faces.iter().enumerate() {
                    let (nx, ny, nz) = neighbors[i];
                    let neighbor_transparent = if nx < 0
                        || nx >= CHUNK_W as i32
                        || ny < 0
                        || ny >= CHUNK_H as i32
                        || nz < 0
                        || nz >= CHUNK_D as i32
                    {
                        true
                    } else {
                        let nid = chunk.get(nx as usize, ny as usize, nz as usize);
                        nid == 0 || (BLOCK_TRANSPARENT[nid as usize] && nid != block_id)
                    };

                    if !neighbor_transparent {
                        continue;
                    }

                    let base = positions.len() as u32;
                    for (vi, v) in verts.iter().enumerate() {
                        positions.push(*v);
                        normals.push(*normal);
                        uvs.push(face_uvs[vi]);
                    }
                    indices.extend_from_slice(&[
                        base,
                        base + 1,
                        base + 2,
                        base,
                        base + 2,
                        base + 3,
                    ]);
                }
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
