use crate::core::block::BlockRegistry;
use crate::voxel::meshing::generate_chunk_mesh;
use crate::world::chunk::{Chunk, NeedsMesh};
use bevy::prelude::*;

/// Generate Needed Meshes
pub fn mesh_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    block_registry: Res<BlockRegistry>,
    mut query: Query<(Entity, &mut Chunk), With<NeedsMesh>>,
) {
    for (entity, mut chunk) in query.iter_mut() {

        if !chunk.dirty {
            commands.entity(entity).remove::<NeedsMesh>();
            continue;
        }

        let mesh = generate_chunk_mesh(&chunk, &block_registry);
        let mesh_handle = meshes.add(mesh);
        
        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 0.8,
            reflectance: 0.2,
            ..default()
        });

        // Mesh and Material
        commands.entity(entity).insert(Mesh3d(mesh_handle));
        commands.entity(entity).insert(MeshMaterial3d(material));
        commands.entity(entity).remove::<NeedsMesh>();


        info!("Generated mesh for chunk at {:?}", chunk.pos);
    }
}

/// Mark Dirty Chunks
pub fn mark_dirty_chunks(
    mut commands: Commands,
    query: Query<(Entity, &Chunk), (Without<NeedsMesh>, Changed<Chunk>)>,
) {
    for (entity, chunk) in query.iter() {
        if chunk.dirty {
            commands.entity(entity).insert(NeedsMesh);
        }
    }
}

/// Mark Initial Chunks for Meshing
pub fn mark_initial_chunks(
    mut commands: Commands,
    query: Query<Entity, (With<Chunk>, Without<NeedsMesh>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(NeedsMesh);
    }
}