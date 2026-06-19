use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod console;
mod info;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(console::ConsolePlugin)
            .add_plugins(info::InfoPlugin);
    }
}
