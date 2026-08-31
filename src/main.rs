use avian3d::PhysicsPlugins;
use bevy::prelude::*;

use crate::scenes::Scene::DevSource;

mod dev_tools;

mod appstate;

mod scenes;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(dev_tools::DevToolsPlugin)
        .add_plugins(appstate::UIPlugin)
        .add_plugins(scenes::ScenesPlugin { scene: DevSource })
        .run();
}
