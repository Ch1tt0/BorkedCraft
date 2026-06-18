use bevy::prelude::*;

mod console;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIState {
    #[default]
    MainMenu,
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UIState>(); // Initialize UIState
        app.add_plugins(console::ConsolePlugin);
        app.add_systems(Update, setup);
    }
}

fn setup() {}
