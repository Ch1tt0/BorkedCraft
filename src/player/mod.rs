use avian3d::prelude::*;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

mod camera;

/// Holds the constants equivalent to pm_accelerate, pm_friction, etc.
#[derive(Component)]
pub struct SourceController {
    pub acceleration: f32,     // pm_accelerate
    pub air_acceleration: f32, // pm_airaccelerate
    pub friction: f32,         // pm_friction
    pub stop_speed: f32,       // pm_stopspeed
    pub max_ground_speed: f32, // wishspeed (ground)
    pub max_air_speed: f32,    // wishspeed (air) - restricted to force strafing
    pub jump_force: f32,       // jump_velocity
}

impl Default for SourceController {
    fn default() -> Self {
        Self {
            acceleration: 10.0,
            air_acceleration: 150.0, // High air acceleration enables aggressive air strafing
            friction: 6.0,
            stop_speed: 100.0,
            max_ground_speed: 320.0,
            max_air_speed: 30.0, // Hard limit on straightforward air speed
            jump_force: 6.0,
        }
    }
}

/// Stores the current movement intent and camera orientation
#[derive(Component)]
pub struct PlayerState {
    pub wish_dir: Vec3,
    pub is_grounded: bool,
    pub jump_queued: bool,
}

fn setup_environment(mut commands: Commands) {
    // 3. Spawn the Player
    let player_height = 1.0;
    let player_radius = 0.4;
    let shape_caster_radius = player_radius * 0.9; // Slightly smaller to prevent wall clinging

    let player_entity = commands
        .spawn((
            // Physics definitions
            RigidBody::Dynamic,
            Collider::capsule(player_radius, player_height),
            LockedAxes::ROTATION_LOCKED, // Prevent the capsule from tipping over
            Friction::ZERO, // Disables material friction (crucial for PM_SlideMove sliding)
            Restitution::ZERO, // Disables bouncing
            LinearVelocity::ZERO,
            AngularVelocity::ZERO,
            GravityScale(1.0),
            SweptCcd::default(), // Prevent tunneling at high speeds
            // Ground detection spatial query
            ShapeCaster::new(
                Collider::sphere(shape_caster_radius),
                Vec3::new(0.0, -(player_height / 2.0) + shape_caster_radius, 0.0),
                Quat::IDENTITY,
                Dir3::NEG_Y,
            )
            .with_max_distance(0.2) // Snapping distance
            .with_query_filter(SpatialQueryFilter::default()), // We will exclude the player entity in the setup
            // Custom logic components
            SourceController::default(),
            PlayerState {
                wish_dir: Vec3::ZERO,
                is_grounded: false,
                jump_queued: false,
            },
            Transform::from_xyz(0.0, 3.0, 0.0),
        ))
        .id();

    // 4. Update the ShapeCaster filter to ignore the player itself
    commands.entity(player_entity).insert(
        ShapeCaster::new(
            Collider::sphere(shape_caster_radius),
            Vec3::new(0.0, -(player_height / 2.0) + shape_caster_radius, 0.0),
            Quat::IDENTITY,
            Dir3::NEG_Y,
        )
        .with_max_distance(0.2)
        .with_query_filter(SpatialQueryFilter::from_excluded_entities([player_entity])),
    );

    // 5. Spawn the Camera as a child
    commands.entity(player_entity).with_children(|parent| {
        parent.spawn(camera::setup());
    });
}

pub struct SourceMovementPlugin;

impl Plugin for SourceMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_cursor, handle_input, rotate_camera))
            // The movement physics strictly belongs in FixedUpdate for determinism
            .add_systems(
                FixedUpdate,
                (
                    ground_detection,
                    apply_friction,
                    apply_acceleration,
                    handle_jumping,
                )
                    .chain(),
            );
    }
}

fn toggle_cursor(
    mut cursor_options: Single<&mut CursorOptions, With<Window>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Middle) {
        cursor_options.visible = !cursor_options.visible;
        cursor_options.grab_mode = match cursor_options.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&PlayerCamera>,
    mut player_query: Query<&mut PlayerState>,
) {
    let Ok(mut state) = player_query.single_mut() else {
        return;
    };
    let Ok(camera) = camera_query.single() else {
        return;
    };

    // 1. Gather raw keyboard input
    let mut local_input = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        local_input.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        local_input.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        local_input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        local_input.x += 1.0;
    }

    // 2. Normalize input to prevent diagonal speed boosting (root 2 anomaly)
    local_input = local_input.normalize_or_zero();

    // 3. Rotate the local input vector by the camera's YAW (horizontal rotation)
    // We explicitly ignore PITCH so looking down doesn't slow forward momentum.
    let yaw_rot = Quat::from_rotation_y(camera.yaw);
    state.wish_dir = yaw_rot * local_input;

    // 4. Register jumps
    if keyboard.just_pressed(KeyCode::Space) {
        state.jump_queued = true;
    }
}

fn rotate_camera(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut PlayerCamera, &mut Transform)>,
) {
    let sensitivity = 0.002;
    for (mut camera, mut transform) in query.iter_mut() {
        camera.yaw -= mouse_motion.delta.x * sensitivity;
        camera.pitch -= mouse_motion.delta.y * sensitivity;

        // Clamp pitch to prevent breaking your neck
        camera.pitch = camera.pitch.clamp(-1.5, 1.5);

        // Apply to transform
        transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    }
}

fn ground_detection(mut query: Query<(&mut PlayerState, &ShapeHits)>) {
    for (mut state, hits) in query.iter_mut() {
        state.is_grounded = false;

        // Check the spatial query results
        if let Some(first_hit) = hits.iter().next() {
            let normal = first_hit.normal1; // Avian3D hit normal

            // Check if the surface is flat enough to walk on (e.g. angle with UP vector)
            // cos(45 degrees) is approx 0.707
            if normal.dot(Vec3::Y) >= 0.707 {
                state.is_grounded = true;
            }
        }
    }
}

fn apply_friction(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &PlayerState, &SourceController)>,
) {
    let dt = time.delta_secs();

    for (mut velocity, state, controller) in query.iter_mut() {
        // Only apply friction if we are on the ground and not about to jump
        if !state.is_grounded || state.jump_queued {
            continue;
        }

        // 1. Isolate the horizontal velocity
        let v_hz = Vec3::new(velocity.x, 0.0, velocity.z);
        let speed = v_hz.length();

        // 2. Micro-drift cutoff
        if speed < 0.1 {
            velocity.x = 0.0;
            velocity.z = 0.0;
            continue;
        }

        // 3. Determine control speed (utilizes stop_speed)
        let control = speed.max(controller.stop_speed);

        // 4. Calculate drop
        let drop = control * controller.friction * dt;

        // 5. Calculate new speed scalar
        let new_speed = (speed - drop).max(0.0) / speed;

        // 6. Apply to the rigid body
        velocity.x = v_hz.x * new_speed;
        velocity.z = v_hz.z * new_speed;
    }
}

fn apply_acceleration(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &PlayerState, &SourceController)>,
) {
    let dt = time.delta_secs();

    for (mut velocity, state, controller) in query.iter_mut() {
        // Choose context-dependent scalars
        let (wish_speed, accel) = if state.is_grounded {
            (controller.max_ground_speed, controller.acceleration)
        } else {
            (controller.max_air_speed, controller.air_acceleration)
        };

        // 1. Calculate the magnitude of current velocity ALONG the wish direction (Vector Projection)
        let current_speed = velocity.dot(state.wish_dir);

        // 2. Calculate remaining capacity
        let add_speed = wish_speed - current_speed;

        // 3. Bailout if capacity is met or exceeded (Crucial for Air Strafing limit break)
        if add_speed <= 0.0 {
            continue;
        }

        // 4. Calculate raw acceleration magnitude
        let mut accel_magnitude = accel * wish_speed * dt;

        // 5. Clamp to the remaining allowed speed
        accel_magnitude = accel_magnitude.min(add_speed);

        // 6. Integrate directly into the LinearVelocity struct.
        // Notice this bypasses external forces entirely.
        velocity.x += state.wish_dir.x * accel_magnitude;
        velocity.z += state.wish_dir.z * accel_magnitude;
    }
}

fn handle_jumping(mut query: Query<(&mut LinearVelocity, &mut PlayerState, &SourceController)>) {
    for (mut velocity, mut state, controller) in query.iter_mut() {
        if state.jump_queued {
            // Only jump if actually grounded
            if state.is_grounded {
                // Direct overwrite of the Y axis vector
                velocity.y = controller.jump_force;
                state.is_grounded = false; // Immediately invalidate ground state
            }

            // Consume the jump input queue regardless of success
            state.jump_queued = false;
        }
    }
}
