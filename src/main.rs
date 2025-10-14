use bevy::prelude::*;
use bevy::window::WindowResolution;
use aeternitas::core::block::BlockRegistry;
use aeternitas::player::controller::*;
use aeternitas::voxel::rendering::*;
use aeternitas::world::chunk_manager::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aeternitas".to_string(),
                resolution: WindowResolution::from((1920, 1080)),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .init_resource::<ChunkManager>()
        .insert_resource(BlockRegistry::new())
        // Startup
        .add_systems(Startup, (
            setup_camera,
            setup_lighting,
            spawn_initial_chunks,
        ))
        // Update
        .add_systems(Update, (
            camera_movement,
            camera_look,
            mark_initial_chunks,
            mark_dirty_chunks,
            mesh_chunks,
        ))
        .run();
}

fn setup_lighting(mut commands: Commands) {
    // Temporary Sunlight
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -0.7,
            0.5,
            0.0,
        )),
    ));
}