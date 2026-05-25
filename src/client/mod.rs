use bevy::prelude::*;

use crate::game::player::components::{Player, PlayerInput};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_camera)
            .add_systems(Startup, spawn_debug_light)
            // TODO: replace the offline keyboard detection with
            // general local input detection to fit into the
            // multiplayer and PlayerInput system
            .add_systems(PreUpdate, capture_client_input);
    }
}

// TODO: Make into FPS camera later
fn spawn_debug_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 9.0, 12.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

// TODO: remove eventually
// TODO: world darkest light level should be pitch black
fn spawn_debug_light(mut cmds: Commands) {
    cmds.spawn((
        DirectionalLight {
            illuminance: 30000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.8, 0.0)),
    ));
}

fn capture_client_input(
    key_in: Res<ButtonInput<KeyCode>>,
    mut player_inputs: Query<&mut PlayerInput, With<Player>>,
) {
    let forward = key_in.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let backward = key_in.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = key_in.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = key_in.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let move_dir = Vec2::new(
        (right as i8 - left as i8) as f32,
        (forward as i8 - backward as i8) as f32,
    )
    .clamp_length_max(1.0);

    for mut player_input in &mut player_inputs {
        player_input.move_dir = move_dir;

        if key_in.just_pressed(KeyCode::Space) {
            player_input.jump_pressed = true;
        }
    }
}
