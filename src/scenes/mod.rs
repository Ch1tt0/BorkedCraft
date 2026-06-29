use ::bevy::prelude::*;

mod dev;

#[derive(Debug, Clone, Copy, Default)]
pub enum Scene {
    #[default]
    Dev,
}

pub struct ScenesPlugin {
    pub scene: Scene,
}

impl Default for ScenesPlugin {
    fn default() -> Self {
        Self {
            scene: Scene::default(),
        }
    }
}

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        match self.scene {
            Scene::Dev => {
                app.add_systems(Startup, dev::scene.spawn());
            }
        }
    }
}
