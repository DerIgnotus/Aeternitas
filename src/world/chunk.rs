use crate::core::{block::BlockId, position::{ChunkPos, LocalPos, CHUNK_SIZE}};
use bevy::prelude::*;
use std::collections::HashMap;

const CHUNK_VOLUME: usize = (CHUNK_SIZE as usize).pow(3);

/// 25x25x25 (May change later)
#[derive(Component)]
pub struct Chunk {
    pub pos: ChunkPos,
    blocks: Box<[BlockId; CHUNK_VOLUME]>,
    // Extra Block Data
    pub block_entities: HashMap<LocalPos, BlockEntity>,
    pub dirty: bool,
    pub depth: u8,
}

impl Chunk {
    /// Empty Chunk
    pub fn empty(pos: ChunkPos, depth: u8) -> Self {
        Self {
            pos,
            blocks: Box::new([BlockId::AIR; CHUNK_VOLUME]),
            block_entities: HashMap::new(),
            dirty: true,
            depth,
        }
    }

pub fn generate_chunk(pos: ChunkPos, depth: u8) -> Self {
    let mut chunk = Self::empty(pos, depth);

    use noise::{NoiseFn, Perlin};
    let perlin = Perlin::new(0);

    let frequency = 0.01;
    let amplitude = 100.0;
    let base_height = -80;

    let scale = 1 << depth;

    // Use chunk's world position as the base — do NOT re-snap it
    let chunk_world_pos = pos.to_world_pos();

    for z in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            // World coordinates, scale each block correctly
            let world_x = chunk_world_pos.x + (x as i32) * scale;
            let world_z = chunk_world_pos.z + (z as i32) * scale;

            let noise_value = perlin.get([world_x as f64 * frequency, world_z as f64 * frequency]);
            let height = ((noise_value + 1.0) * 0.5 * amplitude) as i32 + base_height;

            for y in 0..CHUNK_SIZE {
                let world_y = chunk_world_pos.y + (y as i32) * scale;

                if world_y <= height - 3 {
                    chunk.set_block(LocalPos::new(x, y, z), BlockId::STONE);
                } else if world_y <= height - 1 {
                    chunk.set_block(LocalPos::new(x, y, z), BlockId::DIRT);
                } else if world_y <= height + scale {
                    chunk.set_block(LocalPos::new(x, y, z), BlockId::GRASS);
                }
            }
        }
    }

    chunk
}

    /// Test Chunk
    pub fn test_chunk(pos: ChunkPos, depth: u8) -> Self {
        let mut chunk = Self::empty(pos, depth);
        
        for y in 0..5 {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let local = LocalPos::new(x, y, z);
                    chunk.set_block(local, BlockId::STONE);
                }
            }
        }

        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let local = LocalPos::new(x, 5, z);
                chunk.set_block(local, BlockId::DIRT);
            }
        }

        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let local = LocalPos::new(x, 6, z);
                chunk.set_block(local, BlockId::GRASS);
            }
        }

        chunk
    }

    pub fn get_block(&self, pos: LocalPos) -> BlockId {
        self.blocks[pos.to_index()]
    }

    pub fn set_block(&mut self, pos: LocalPos, block: BlockId) {
        self.blocks[pos.to_index()] = block;
        self.dirty = true;
    }

    /// Iterate
    pub fn iter_blocks(&self) -> impl Iterator<Item = (LocalPos, BlockId)> + '_ {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, id)| **id != BlockId::AIR)
            .map(|(idx, id)| (LocalPos::from_index(idx), *id))
    }
    
}

/// Extra Block Data
#[derive(Debug, Clone)]
pub enum BlockEntity {
    Machine {
        // Will implement later
    },
    MultiblockPart {
        // Will implement later
    },
    Storage {
        // Will implement later
    },
}

/// Marker Component for Meshing
#[derive(Component)]
pub struct NeedsMesh;