use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow, WindowMode}; //Not used due to API problems

/// Flying Camera
#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,
    pub sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 10.0,
            sensitivity: 0.001,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

/// Camera
pub fn setup_camera(mut commands: Commands) {
    let yaw = 0.0;
    let pitch = 0.0;
    
    let rotation = Quat::from_axis_angle(Vec3::Y, yaw)
        * Quat::from_axis_angle(Vec3::X, pitch);
    
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(12.0, 20.0, 12.0)
            .with_rotation(rotation),
        FlyCamera {
            yaw,
            pitch,
            ..Default::default()
        },
    ));
}

/// Camera Movement
pub fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
) {
    for (mut transform, camera) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        
        // Movement Normalization
        let forward = Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize_or_zero();
        let right = Vec3::new(transform.right().x, 0.0, transform.right().z).normalize_or_zero();

        if keys.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }

        if keys.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity += right;
        }

        if keys.pressed(KeyCode::Space) {
            velocity += Vec3::Y;
        }
        if keys.pressed(KeyCode::ShiftLeft) {
            velocity -= Vec3::Y;
        }

        // Sprint
        let speed_multiplier = if keys.pressed(KeyCode::ControlLeft) {
            3.0
        } else {
            1.0
        };

        if velocity.length() > 0.0 {
            velocity = velocity.normalize();
            transform.translation += velocity * camera.speed * speed_multiplier * time.delta_secs();
        }
    }
}

/// Mouse Look
pub fn camera_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    if delta.length_squared() > 0.0 {
        for (mut transform, mut camera) in query.iter_mut() {
            camera.yaw -= delta.x * camera.sensitivity;
            camera.pitch -= delta.y * camera.sensitivity;
            camera.pitch = camera.pitch.clamp(-1.54, 1.54); // ~88 degrees

            // Camera Rotation
            transform.rotation = Quat::from_axis_angle(Vec3::Y, camera.yaw)
                * Quat::from_axis_angle(Vec3::X, camera.pitch);
        }
    }
}