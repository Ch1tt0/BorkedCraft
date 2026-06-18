use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConsoleState {
    #[default]
    Closed,
    Open,
    Closing,
    Opening,
}

#[derive(Component)]
struct Console;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ConsoleState>();
        app.add_systems(Startup, spawn_box);
        app.add_systems(Update, toggle_console);
        app.add_systems(
            Update,
            animate_console
                .run_if(in_state(ConsoleState::Opening).or(in_state(ConsoleState::Closing))),
        );
    }
}

impl ConsoleState {
    fn next(&self) -> ConsoleState {
        match self {
            ConsoleState::Closed => ConsoleState::Opening,
            ConsoleState::Open => ConsoleState::Closing,
            ConsoleState::Closing => ConsoleState::Closed,
            ConsoleState::Opening => ConsoleState::Open,
        }
    }
}

fn spawn_box(mut commands: Commands) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(60.0),
        justify_content: JustifyContent::Start,
        align_content: AlignContent::Start,
        ..default()
    };

    let square = (
        BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
    );

    commands.spawn((container, children![(square)], Console));
}

fn toggle_console(
    mut next_state: ResMut<NextState<ConsoleState>>,
    current_state: Res<State<ConsoleState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Backquote) {
        match current_state.get() {
            ConsoleState::Closed => {
                next_state.set(current_state.next());
            }
            ConsoleState::Open => {
                next_state.set(current_state.next());
            }
            ConsoleState::Closing => {}
            ConsoleState::Opening => {}
        }
    }
}

fn animate_console(
    time: Res<Time>,
    mut query: Query<&Node, With<Console>>,
    mut next_state: ResMut<NextState<ConsoleState>>,
    current_state: Res<State<ConsoleState>>,
) {
    for mut node in &query {

    }
    let target_height = match current_state.get() {
        ConsoleState::Closed => Val::Percent(0.0),
        ConsoleState::Open => Val::Percent(60.0),
        ConsoleState::Closing => Val::Percent(0.0),
        ConsoleState::Opening => Val::Percent(60.0),
    };

    // let current_height = node.
}
