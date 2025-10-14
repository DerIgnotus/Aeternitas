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
}

impl Chunk {
    /// Empty Chunk
    pub fn empty(pos: ChunkPos) -> Self {
        Self {
            pos,
            blocks: Box::new([BlockId::AIR; CHUNK_VOLUME]),
            block_entities: HashMap::new(),
            dirty: true,
        }
    }

    /// Test Chunk
    pub fn test_chunk(pos: ChunkPos) -> Self {
        let mut chunk = Self::empty(pos);
        
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