use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod performance_info;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_plugins(performance_info::PerformanceInfoPlugin);
    }
}
