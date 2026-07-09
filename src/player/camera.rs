use bevy::prelude::*;

pub struct CameraPlugin;

#[derive(Component, Clone)]
struct PlayerCamera;

impl Plugin for CameraPlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, setup_camera.spawn());
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera
    }
}


fn setup_camera() -> impl Scene {
    bsn![(
        #PlayerCamera
        PlayerCamera
        Camera3d
        template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
    )]
}
