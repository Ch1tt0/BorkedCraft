use ::bevy::prelude::*;

mod dev;

mod dev_source;

#[derive(Debug, Clone, Copy, Default)]
pub enum Scene {
    #[default]
    Dev,
    DevSource,
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
            Scene::DevSource => {
                app.add_systems(Startup, dev_source::scene);
            }
        }
    }
}
