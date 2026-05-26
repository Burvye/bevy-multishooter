use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::game::player::components::{Input, LCamAnchor, LocalPlayer, LookState, VisRoot};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_window_for_mouse_look)
            .add_systems(Startup, spawn_debug_light)
            // TODO: replace the offline keyboard detection with
            // general local input detection to fit into the
            // multiplayer and PlayerInput system
            .add_systems(PreUpdate, capture_client_input)
            // camera looking is not updated based on fixed tick for smoothness
            .add_systems(PreUpdate, capture_local_look_input)
            .add_systems(Update, sync_local_player_visuals);
    }
}

#[derive(Component)]
pub struct PlayerCamera;

fn configure_window_for_mouse_look(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
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
    mut player_input: Single<&mut Input, With<LocalPlayer>>,
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

    player_input.move_dir = move_dir;

    if key_in.just_pressed(KeyCode::Space) {
        player_input.jump_pressed = true;
    }
}

fn capture_local_look_input(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut player_look: Single<&mut LookState, With<LocalPlayer>>,
) {
    if mouse_motion.delta == Vec2::ZERO {
        return;
    }

    const MOUSE_SENSITIVITY: f32 = 0.003;

    player_look.yaw -= mouse_motion.delta.x * MOUSE_SENSITIVITY;
    player_look.pitch =
        (player_look.pitch - mouse_motion.delta.y * MOUSE_SENSITIVITY).clamp(-1.57, 1.57);
}

fn sync_local_player_visuals(
    local_player: Single<(&LookState, &Children), With<LocalPlayer>>,
    children_query: Query<&Children>,
    mut visual_root_query: Query<&mut Transform, With<VisRoot>>,
    mut camera_anchor_query: Query<&mut Transform, (With<LCamAnchor>, Without<VisRoot>)>,
) {
    let (look, children) = *local_player;

    for child in children.iter() {
        if let Ok(mut visual_root_transform) = visual_root_query.get_mut(child) {
            // yaw rotation is visible and affects movement input
            // pitch affects only the camera and is lower in hierarchy
            visual_root_transform.rotation = Quat::from_rotation_y(look.yaw);
        }

        if let Ok(visual_root_children) = children_query.get(child) {
            for visual_root_child in visual_root_children.iter() {
                if let Ok(mut camera_anchor_transform) =
                    camera_anchor_query.get_mut(visual_root_child)
                {
                    camera_anchor_transform.rotation = Quat::from_rotation_x(look.pitch);
                }
            }
        }
    }
}
