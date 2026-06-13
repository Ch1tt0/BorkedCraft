use bevy::prelude::*;

mod dev_tools;
use crate::dev_tools::DevToolsPlugin;

mod ui;
use crate::ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DevToolsPlugin)
        .add_plugins(UIPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
