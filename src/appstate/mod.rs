use bevy::prelude::*;
pub struct AppStatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.add_systems(Update, setup);
    }
}

impl AppState {
    fn next(&self) -> Self {
        match *self {
            AppState::MainMenu => AppState::InGame,
            AppState::InGame => AppState::Paused,
            AppState::Paused => AppState::InGame,
        }
    }
}

fn setup() {}
