use bevy::prelude::*;
use crate::systems::camera_follow::camera_follow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
           .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    // Isometric offset: equal distance on all axes → true isometric angle
    let eye = Vec3::new(20.0, 20.0, 20.0);

    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 0.05,
            ..default()
        }
        .into(),
        transform: Transform::from_translation(eye).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
