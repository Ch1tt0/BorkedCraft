use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*, text::FontSourceTemplate,
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
        app.add_systems(Startup, setup_info.spawn());
        app.add_systems(Update, change_visibility);
        app.add_systems(Update, update_info);
    }
}

#[derive(Component, Clone)]
struct InfoContainer;

impl Default for InfoContainer {
    fn default() -> Self {
        InfoContainer
    }
}

#[derive(Component, Clone)]
struct FPSText;

impl Default for FPSText {
    fn default() -> Self {
        FPSText
    }
}

fn setup_info() -> impl Scene {
    bsn! {
        Node {
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Start,
            display: Display::None,
        }
        InfoContainer
        Children[(
        Text::default()
        TextFont {
            font: FontSourceTemplate::Handle("fonts/FiraMonoNerdFont.otf"),
            font_size: FontSize::Rem(1.0),
        }
        FPSText
        )]
    }
}

fn change_visibility(
    mut next_state: ResMut<NextState<InfoState>>,
    current_info_state: Res<State<InfoState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F3) {
        match current_info_state.get() {
            InfoState::Hidden => {
                next_state.set(InfoState::Visible);
            }
            InfoState::Visible => {
                next_state.set(InfoState::Hidden);
            }
        }
    }
}

fn update_info(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FPSText>>,
    mut container_query: Query<&mut Node, With<InfoContainer>>,
    current_info_state: Res<State<InfoState>>,
    current_console_state: Res<State<ConsoleState>>,
) {
    if *current_console_state.get() == ConsoleState::Closed && *current_info_state.get() == InfoState::Visible {
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
