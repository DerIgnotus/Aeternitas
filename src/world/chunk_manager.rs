use crate::core::position::{ChunkPos, CHUNK_SIZE};
use crate::world::chunk::Chunk;
use crate::world::octree::Octree;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// Chunk Manager
#[derive(Resource)]
pub struct ChunkManager {
    pub loaded_chunks: HashMap<ChunkPos, Entity>,
    pub force_loaded: HashSet<ChunkPos>,
    pub render_distance_horizontal: i32,
    pub render_distance_vertical: i32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            loaded_chunks: HashMap::new(),
            force_loaded: HashSet::new(),
            render_distance_horizontal: 16,
            render_distance_vertical: 8,
        }
    }
}

impl ChunkManager {
    pub fn new(render_distance_horizontal: i32) -> Self {
        Self {
            loaded_chunks: HashMap::new(),
            force_loaded: HashSet::new(),
            render_distance_horizontal,
            render_distance_vertical: render_distance_horizontal * 2,
        }
    }

    /// Check Chunks Near Player
    pub fn should_load(&self, chunk_pos: ChunkPos, player_chunk: ChunkPos) -> bool {
        if self.force_loaded.contains(&chunk_pos) {
            return true;
        }

        if self.loaded_chunks.len() > 10000 { warn!("Chunk cap reached, skipping new loads"); return false; }

        let dx = (chunk_pos.x - player_chunk.x).abs();
        let dy = (chunk_pos.y - player_chunk.y).abs();
        let dz = (chunk_pos.z - player_chunk.z).abs();

        dx <= self.render_distance_horizontal
            && dz <= self.render_distance_horizontal
            && dy <= self.render_distance_vertical
    }

    /// Get Chunks Near Player
    pub fn chunks_to_load(&self, player_chunk: ChunkPos) -> Vec<ChunkPos> {
        let mut chunks = Vec::new();

        for x in -self.render_distance_horizontal..=self.render_distance_horizontal {
            for z in -self.render_distance_horizontal..=self.render_distance_horizontal {
                for y in -self.render_distance_vertical..=self.render_distance_vertical {
                    let chunk_pos = ChunkPos::new(
                        player_chunk.x + x,
                        player_chunk.y + y,
                        player_chunk.z + z,
                    );
                    chunks.push(chunk_pos);
                }
            }
        }

        chunks
    }

    pub fn is_loaded(&self, pos: ChunkPos) -> bool {
        self.loaded_chunks.contains_key(&pos)
    }

    pub fn register_chunk(&mut self, pos: ChunkPos, entity: Entity) {
        self.loaded_chunks.insert(pos, entity);
    }

    pub fn unregister_chunk(&mut self, pos: ChunkPos) {
        self.loaded_chunks.remove(&pos);
    }

    pub fn get_chunk_entity(&self, pos: ChunkPos) -> Option<Entity> {
        self.loaded_chunks.get(&pos).copied()
    }
}

pub fn update_chunks_around_player(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    camera_query: Query<&Transform, With<Camera>>,
    mut octree: ResMut<Octree>,
) {
    if let Ok(camera_transform) = camera_query.single() {

        // Subdivide octree nodes near the player (in world coordinates)
        octree.subdivide_near_player(camera_transform.translation);

        // Collect leaf nodes
        let leaves = octree.collect_leaves();

        // Convert leaf nodes directly to chunk positions in grid coordinates
        let leaf_positions: Vec<ChunkPos> = leaves
            .iter()
            .map(|node| {
                // Compute node's minimum corner in grid space
                let node_min = node.center - Vec3::splat(node.size / 2.0);
                ChunkPos::from_world_pos(node_min)
            })
            .collect();

        for node in &leaves {
            let node_min = node.center - Vec3::splat(node.size / 2.0);
            let chunk_pos = ChunkPos::from_world_pos(node_min);

            if !chunk_manager.is_loaded(chunk_pos) {
                // Compute LOD based on node size relative to base CHUNK_SIZE
                let lod = (node.size / CHUNK_SIZE as f32).log2() as u8;

                let chunk = Chunk::generate_chunk(chunk_pos, lod);

                let entity = commands.spawn((
                    chunk,
                    Transform::from_translation(node_min),
                    GlobalTransform::default(),
                )).id();

                chunk_manager.register_chunk(chunk_pos, entity);
                info!("Spawned chunk at {:?} (size {:.1}, lod {})", node_min, node.size, lod);
            }
        }

        // Unload chunks that are no longer in leaves
        let mut to_unload = Vec::new();
        for (&pos, &entity) in chunk_manager.loaded_chunks.iter() {
            if !leaf_positions.contains(&pos) {
                to_unload.push((pos, entity));
            }
        }

        for (pos, entity) in to_unload {
            commands.entity(entity).despawn();
            chunk_manager.unregister_chunk(pos);
            info!("Unloaded chunk at {:?}", pos);
        }
    }
}

/// Test Chunks
pub fn spawn_initial_chunks(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
) {

    let chunk_pos = ChunkPos::new(0, 0, 0);
    let chunk = Chunk::test_chunk(chunk_pos, 0);
    
    let entity = commands.spawn((
        chunk,
        Transform::from_translation(chunk_pos.to_world_pos().to_vec3()),
        GlobalTransform::default(),
    )).id();
    
    chunk_manager.register_chunk(chunk_pos, entity);
    
    info!("Spawned test chunk at {:?}", chunk_pos);
}