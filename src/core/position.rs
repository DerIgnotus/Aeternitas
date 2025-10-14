use bevy::prelude::*;
use std::ops::Add;

/// World Wrap
pub const WORLD_SIZE: i32 = 1000;
pub const WORLD_HEIGHT: i32 = 1000;
pub const CHUNK_SIZE: u8 = 25;
pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

/// Chunk Position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32, 
    pub z: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: Self::wrap_horizontal(x),
            y: y.clamp(-20, 19),
            z: Self::wrap_horizontal(z),
        }
    }

    /// Coordinate Wrapping
    fn wrap_horizontal(coord: i32) -> i32 {
        let half = WORLD_SIZE / 2;
        ((coord + half).rem_euclid(WORLD_SIZE)) - half
    }

    /// World Position
    pub fn to_world_pos(&self) -> BlockPos {
        BlockPos::new(
            self.x * CHUNK_SIZE_I32,
            self.y * CHUNK_SIZE_I32,
            self.z * CHUNK_SIZE_I32,
        )
    }

    /// Neighbor Chunk Positions
    pub fn neighbors(&self) -> [ChunkPos; 6] {
        [
            ChunkPos::new(self.x + 1, self.y, self.z),
            ChunkPos::new(self.x - 1, self.y, self.z),
            ChunkPos::new(self.x, self.y + 1, self.z),
            ChunkPos::new(self.x, self.y - 1, self.z),
            ChunkPos::new(self.x, self.y, self.z + 1),
            ChunkPos::new(self.x, self.y, self.z - 1),
        ]
    }
}

/// Global Block Positions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x: ChunkPos::wrap_horizontal(x),
            y: y.clamp(-500, 499),
            z: ChunkPos::wrap_horizontal(z),
        }
    }

    /// Chunk Containing This Block
    pub fn chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(
            self.x.div_euclid(CHUNK_SIZE_I32),
            self.y.div_euclid(CHUNK_SIZE_I32),
            self.z.div_euclid(CHUNK_SIZE_I32),
        )
    }

    /// Local Position In Chunk
    pub fn local_pos(&self) -> LocalPos {
        LocalPos {
            x: self.x.rem_euclid(CHUNK_SIZE_I32) as u8,
            y: self.y.rem_euclid(CHUNK_SIZE_I32) as u8,
            z: self.z.rem_euclid(CHUNK_SIZE_I32) as u8,
        }
    }

    /// Bevy Vec3 Conversion
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl Add<IVec3> for BlockPos {
    type Output = BlockPos;
    fn add(self, offset: IVec3) -> BlockPos {
        BlockPos::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}

/// Local Position Within Chunk
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalPos {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl LocalPos {
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < CHUNK_SIZE && y < CHUNK_SIZE && z < CHUNK_SIZE);
        Self { x, y, z }
    }

    /// Array Index for Chunks
    pub fn to_index(&self) -> usize {
        (self.y as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize)
            + (self.z as usize * CHUNK_SIZE as usize)
            + self.x as usize
    }

    /// Array Index to Local Position
    pub fn from_index(idx: usize) -> Self {
        let cs = CHUNK_SIZE as usize;
        let y = (idx / (cs * cs)) as u8;
        let z = ((idx % (cs * cs)) / cs) as u8;
        let x = (idx % cs) as u8;
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_wrapping() {
        let pos = ChunkPos::new(501, 0, -501);
        assert_eq!(pos.x, -499);
        assert_eq!(pos.z, 499);
    }

    #[test]
    fn test_block_to_chunk() {
        let block = BlockPos::new(50, 25, -30);
        let chunk = block.chunk_pos();
        assert_eq!(chunk, ChunkPos::new(2, 1, -2));
    }

    #[test]
    fn test_local_pos_index() {
        let local = LocalPos::new(5, 10, 15);
        let idx = local.to_index();
        let recovered = LocalPos::from_index(idx);
        assert_eq!(local, recovered);
    }
}