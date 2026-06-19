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
        app.add_systems(Startup, spawn_console);
        app.add_systems(Update, toggle_console);
        app.add_systems(
            Update,
            animate_console
                .run_if(in_state(ConsoleState::Opening).or(in_state(ConsoleState::Closing))),
        ); // Only run animate_console when the console is opening or closing.
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

fn spawn_console(mut commands: Commands) {
    let container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(0.0),
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
            _ => {}
        }
    }
}

fn animate_console(
    time: Res<Time>,
    mut query: Query<&mut Node, With<Console>>,
    mut next_state: ResMut<NextState<ConsoleState>>,
    current_state: Res<State<ConsoleState>>,
) {
    let state = current_state.get();

    // Speed in percentage points per second.
    // (e.g., 200% per second takes exactly 0.3 seconds to reach 60%)
    let animation_speed = 200.0;

    for mut node in &mut query {
        // Extract the current height percentage.
        if let Val::Percent(mut current_height) = node.height {
            match state {
                ConsoleState::Opening => {
                    current_height += animation_speed * time.delta_secs();

                    if current_height >= 60.0 {
                        current_height = 60.0;
                        next_state.set(current_state.next()); // Change state when done!
                    }
                }
                ConsoleState::Closing => {
                    current_height -= animation_speed * time.delta_secs();

                    if current_height <= 0.0 {
                        current_height = 0.0;
                        next_state.set(current_state.next()); // Change state when done!
                    }
                }
                _ => {}
            }

            // Apply the updated height back to the UI style.
            node.height = Val::Percent(current_height);
        }
    }
}
