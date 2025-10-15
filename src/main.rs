use aeternitas::core::block::BlockRegistry;
use aeternitas::player::controller::*;
use aeternitas::voxel::rendering::*;
use aeternitas::world::chunk_manager::*;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, WindowResolution};

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
        .add_systems(
            Startup,
            (setup_camera, setup_lighting, cursor),
        )
        // Update
        .add_systems(
            Update,
            (
                camera_movement,
                camera_look,
                update_chunks_around_player,
                //mark_initial_chunks, // only used for test chunks
                mark_dirty_chunks,
                mesh_chunks,
                exit_system,
            ),
        )
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
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.5, 0.0)),
    ));
}

fn cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn exit_system(mut exit: EventWriter<AppExit>, keys: Res<ButtonInput<KeyCode>>) {
    if (keys.pressed(KeyCode::Escape)) {
        exit.write(AppExit::Success);
    }
}
