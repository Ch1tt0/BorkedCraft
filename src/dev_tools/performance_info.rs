use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct PerformanceInfoPlugin;

impl Plugin for PerformanceInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_stats);
    }
}

#[derive(Component)]
struct FPSText;

// TODO: Implement loading the font into a global resource when starting the game.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font: asset_server.load("fonts/FiraMonoNerdFont.otf"),
                font_size: 16.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font: asset_server.load("fonts/FiraMonoNerdFont.otf"),
                font_size: 16.0,
                ..default()
            },
            FPSText,
        ));
}

fn update_stats(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut TextSpan, With<FPSText>>,
) {
    for mut fps_text in &mut fps_text_query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            **fps_text = format!("{value:.0}");
        }
    }
}
