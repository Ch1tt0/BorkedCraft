use bevy::prelude::*;

mod dev_tools;
use crate::dev_tools::DevToolsPlugin;

mod ui;
use crate::ui::UIPlugin;

mod scenes;
use crate::scenes::ScenesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DevToolsPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(ScenesPlugin::default())
        .run();
}
