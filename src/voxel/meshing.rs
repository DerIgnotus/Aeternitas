use crate::core::{block::{BlockId, BlockRegistry}, position::{LocalPos, CHUNK_SIZE}};
use crate::world::chunk::Chunk;
use bevy::prelude::*;

/// Mesh Generation (Not Optimized)
pub fn generate_chunk_mesh(chunk: &Chunk, block_registry: &BlockRegistry) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    let lod_level = chunk.depth;
    let step = 1 << lod_level;
    
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let block_id = chunk.get_block(LocalPos::new(x as u8, y as u8, z as u8));
                if block_registry.get_or_air(block_id).is_solid {
                    add_block_faces(
                        LocalPos::new(x as u8, y as u8, z as u8),
                        block_id,
                        chunk,
                        block_registry,
                        step as f32,
                        &mut positions,
                        &mut normals,
                        &mut uvs,
                        &mut colors,
                        &mut indices,
                    );
                }
            }
        }
    }


    // Build Mesh
    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        default(),
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(bevy::mesh::Indices::U32(indices));

    mesh
}

fn add_block_faces(
    pos: LocalPos,
    block_id: BlockId,
    chunk: &Chunk,
    block_registry: &BlockRegistry,
    scale: f32,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
) {
    let block_props = block_registry.get_or_air(block_id);
    let color: [f32; 4] = block_props.debug_color.to_srgba().to_f32_array();

    let x = pos.x as f32 * scale;
    let y = pos.y as f32 * scale;
    let z = pos.z as f32 * scale;

    let faces = [
        (
            [0.0, 1.0, 0.0], // Up
            should_render_face(chunk, pos, 0, 1, 0),
            [
                [x, y + scale, z],
                [x + scale, y + scale, z],
                [x + scale, y + scale, z + scale],
                [x, y + scale, z + scale],
            ],
        ),
        (
            [0.0, -1.0, 0.0], // Down
            should_render_face(chunk, pos, 0, -1, 0),
            [
                [x, y, z + scale],
                [x + scale, y, z + scale],
                [x + scale, y, z],
                [x, y, z],
            ],
        ),
        (
            [0.0, 0.0, 1.0], // North (+Z)
            should_render_face(chunk, pos, 0, 0, 1),
            [
                [x, y, z + scale],
                [x, y + scale, z + scale],
                [x + scale, y + scale, z + scale],
                [x + scale, y, z + scale],
            ],
        ),
        (
            [0.0, 0.0, -1.0], // South (-Z)
            should_render_face(chunk, pos, 0, 0, -1),
            [
                [x + scale, y, z],
                [x + scale, y + scale, z],
                [x, y + scale, z],
                [x, y, z],
            ],
        ),
        (
            [1.0, 0.0, 0.0], // East (+X)
            should_render_face(chunk, pos, 1, 0, 0),
            [
                [x + scale, y, z + scale],
                [x + scale, y + scale, z + scale],
                [x + scale, y + scale, z],
                [x + scale, y, z],
            ],
        ),
        (
            [-1.0, 0.0, 0.0], // West (-X)
            should_render_face(chunk, pos, -1, 0, 0),
            [
                [x, y, z],
                [x, y + scale, z],
                [x, y + scale, z + scale],
                [x, y, z + scale],
            ],
        ),
    ];

    for (normal, should_render, verts) in faces {
        if should_render {
            let base_idx = positions.len() as u32;

            for vert in verts {
                positions.push(vert);
                normals.push(normal);
                uvs.push([0.0, 0.0]);
                colors.push(color);
            }

            indices.extend_from_slice(&[
                base_idx, base_idx + 2, base_idx + 1,
                base_idx, base_idx + 3, base_idx + 2,
            ]);
        }
    }
}


///Chunk Render
fn should_render_face(chunk: &Chunk, pos: LocalPos, dx: i32, dy: i32, dz: i32) -> bool {
    let nx = pos.x as i32 + dx;
    let ny = pos.y as i32 + dy;
    let nz = pos.z as i32 + dz;

    // Out-of-bounds faces are always visible
    if nx < 0 || nx >= CHUNK_SIZE as i32
        || ny < 0 || ny >= CHUNK_SIZE as i32
        || nz < 0 || nz >= CHUNK_SIZE as i32
    {
        return true;
    }

    let neighbor_pos = LocalPos::new(nx as u8, ny as u8, nz as u8);
    let neighbor_block = chunk.get_block(neighbor_pos);

    // Consider a block “empty” if neighbor is AIR or this LOD step skips multiple blocks
    neighbor_block == BlockId::AIR
}