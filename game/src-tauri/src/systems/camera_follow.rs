use bevy::prelude::*;
use crate::components::CameraTarget;

/// Smoothly moves the camera to stay above the CameraTarget entity.
pub fn camera_follow(
    target: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let Ok(target_tf) = target.get_single() else { return };
    let Ok(mut cam_tf) = camera.get_single_mut() else { return };

    // Offset in world space: camera sits above-behind the target in isometric space
    let offset = Vec3::new(20.0, 20.0, 20.0);
    let desired = target_tf.translation + offset;
    cam_tf.translation = cam_tf.translation.lerp(desired, 0.1);
}
