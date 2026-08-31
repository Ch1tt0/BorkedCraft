use bevy::prelude::*;

#[derive(Component, Clone)]
struct PlayerCamera {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

fn setup() -> impl Scene {
    bsn![(
        #PlayerCamera
        Name("PlayerCamera")
        Camera3d
        PlayerCamera {
            yaw: 0.0,
            pitch: 0.0,
        }
        Transform::from_xyz(0.0, 1.0 / 2.0 + 0.2, 0.0)
    )]
}
