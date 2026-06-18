use bevy::prelude::*;

mod dev_tools;

mod ui;

mod scenes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(dev_tools::DevToolsPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(scenes::ScenesPlugin::default())
        .run();
}
