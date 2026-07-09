use ::bevy::prelude::*;

mod dev;

#[derive(Debug, Clone, Copy, Default)]
pub enum Scene {
    #[default]
    Dev,
}
#[derive(Default)]
pub struct ScenesPlugin {
    pub scene: Scene,
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
