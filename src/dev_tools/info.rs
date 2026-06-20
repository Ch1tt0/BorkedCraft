use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::dev_tools::console::ConsoleState;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InfoState {
    #[default]
    Hidden,
    Visible,
}

pub struct InfoPlugin;

impl Plugin for InfoPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<InfoState>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, change_visibility);
        app.add_systems(Update, update_info);
    }
}

impl InfoState {
    fn next(&self) -> InfoState {
        match self {
            InfoState::Hidden => InfoState::Visible,
            InfoState::Visible => InfoState::Hidden,
        }
    }
}

#[derive(Component)]
struct InfoContainer;

#[derive(Component)]
struct FPSText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let container = Node {
        justify_content: JustifyContent::Start,
        align_content: AlignContent::Start,
        display: Display::None,
        ..default()
    };

    let text = (
        Text::default(),
        TextFont {
            font: FontSource::Handle(asset_server.load("fonts/FiraMonoNerdFont.otf")),
            font_size: FontSize::Rem(1.0),
            ..default()
        },
        FPSText,
    );
    commands.spawn((container, children![(text)], InfoContainer));
}

fn change_visibility(
    mut next_state: ResMut<NextState<InfoState>>,
    current_info_state: Res<State<InfoState>>,
    current_console_state: Res<State<ConsoleState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F3) {
        match current_info_state.get() {
            InfoState::Hidden => {
                next_state.set(current_info_state.next());
            }
            InfoState::Visible => {
                next_state.set(current_info_state.next());
            }
        }
    }

    if *current_console_state.get() != ConsoleState::Closed {
        next_state.set(InfoState::Hidden); // TODO: Fix this hack to hide info when console is open. Maybe use a run_if instead?
    }
}

fn update_info(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FPSText>>,
    mut container_query: Query<&mut Node, With<InfoContainer>>,
    current_info_state: Res<State<InfoState>>,
) {
    if *current_info_state.get() == InfoState::Visible {
        // Update the FPS text with the current FPS value.
        for mut fps_text in &mut fps_text_query {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
                && let Some(value) = fps.smoothed()
            {
                **fps_text = format!("FPS: {value:.0}");
            }
        }

        for mut container in &mut container_query {
            container.display = Display::Flex;
        }
    } else {
        for mut container in &mut container_query {
            container.display = Display::None;
        }
    }
}
