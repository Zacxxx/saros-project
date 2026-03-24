use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use super::chunk::{Chunk, CHUNK_W, CHUNK_D, CHUNK_H};

/// Generates a mesh from visible (non-air) block faces.
pub fn build_chunk_mesh(chunk: &Chunk, cx: i32, cz: i32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals:   Vec<[f32; 3]> = Vec::new();
    let mut indices:   Vec<u32>      = Vec::new();

    let ox = (cx * CHUNK_W as i32) as f32;
    let oz = (cz * CHUNK_D as i32) as f32;

    for y in 0..CHUNK_H {
        for z in 0..CHUNK_D {
            for x in 0..CHUNK_W {
                if chunk.get(x, y, z) == 0 { continue; }

                let fx = ox + x as f32;
                let fy = y as f32;
                let fz = oz + z as f32;

                // Emit each face only if the neighbour is air
                let faces: &[([f32;3], [f32;3], [[f32;3];4])] = &[
                    // +Y top
                    ([0.,1.,0.], [0.,1.,0.], [[fx,fy+1.,fz],[fx+1.,fy+1.,fz],[fx+1.,fy+1.,fz+1.],[fx,fy+1.,fz+1.]]),
                    // -Y bottom
                    ([0.,-1.,0.], [0.,-1.,0.], [[fx,fy,fz+1.],[fx+1.,fy,fz+1.],[fx+1.,fy,fz],[fx,fy,fz]]),
                    // +X right
                    ([1.,0.,0.], [1.,0.,0.], [[fx+1.,fy,fz],[fx+1.,fy+1.,fz],[fx+1.,fy+1.,fz+1.],[fx+1.,fy,fz+1.]]),
                    // -X left
                    ([-1.,0.,0.], [-1.,0.,0.], [[fx,fy,fz+1.],[fx,fy+1.,fz+1.],[fx,fy+1.,fz],[fx,fy,fz]]),
                    // +Z front
                    ([0.,0.,1.], [0.,0.,1.], [[fx,fy,fz+1.],[fx+1.,fy,fz+1.],[fx+1.,fy+1.,fz+1.],[fx,fy+1.,fz+1.]]),
                    // -Z back
                    ([0.,0.,-1.], [0.,0.,-1.], [[fx+1.,fy,fz],[fx,fy,fz],[fx,fy+1.,fz],[fx+1.,fy+1.,fz]]),
                ];

                let neighbors: [(i32,i32,i32); 6] = [
                    (x as i32, y as i32+1, z as i32),
                    (x as i32, y as i32-1, z as i32),
                    (x as i32+1, y as i32, z as i32),
                    (x as i32-1, y as i32, z as i32),
                    (x as i32, y as i32, z as i32+1),
                    (x as i32, y as i32, z as i32-1),
                ];

                for (i, (_, normal, verts)) in faces.iter().enumerate() {
                    let (nx, ny, nz) = neighbors[i];
                    let exposed = nx < 0 || nx >= CHUNK_W as i32
                        || ny < 0 || ny >= CHUNK_H as i32
                        || nz < 0 || nz >= CHUNK_D as i32
                        || chunk.get(nx as usize, ny as usize, nz as usize) == 0;

                    if !exposed { continue; }

                    let base = positions.len() as u32;
                    for v in verts { positions.push(*v); normals.push(*normal); }
                    indices.extend_from_slice(&[base,base+1,base+2, base,base+2,base+3]);
                }
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}
