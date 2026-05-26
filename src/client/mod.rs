use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

use crate::game::player::components::{
    LocalPlayer, Player, PlayerCameraAnchor, PlayerInput, PlayerLook, PlayerVisualRoot,
};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_window_for_mouse_look)
            .add_systems(Startup, spawn_debug_camera)
            .add_systems(Startup, spawn_debug_light)
            // TODO: replace the offline keyboard detection with
            // general local input detection to fit into the
            // multiplayer and PlayerInput system
            .add_systems(PreUpdate, capture_client_input)
            .add_systems(PreUpdate, capture_local_look_input)
            .add_systems(Update, sync_local_player_visuals)
            .add_systems(Update, sync_camera_to_player_anchor);
    }
}

#[derive(Component)]
struct PlayerCamera;

fn configure_window_for_mouse_look(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

// TODO: Replace this standalone camera with a more explicit first-person
// presentation/camera plugin once the player view pipeline stabilizes.
fn spawn_debug_camera(mut cmds: Commands) {
    cmds.spawn((
        PlayerCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
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

fn capture_local_look_input(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut players: Query<&mut PlayerLook, With<LocalPlayer>>,
) {
    if mouse_motion.delta == Vec2::ZERO {
        return;
    }

    const MOUSE_SENSITIVITY: f32 = 0.003;

    for mut look in &mut players {
        look.yaw -= mouse_motion.delta.x * MOUSE_SENSITIVITY;
        look.pitch = (look.pitch - mouse_motion.delta.y * MOUSE_SENSITIVITY)
            .clamp(-1.45, 1.45);
    }
}

fn sync_local_player_visuals(
    local_player_query: Query<(&PlayerLook, &Children), With<LocalPlayer>>,
    children_query: Query<&Children>,
    mut visual_root_query: Query<&mut Transform, With<PlayerVisualRoot>>,
    mut camera_anchor_query: Query<&mut Transform, (With<PlayerCameraAnchor>, Without<PlayerVisualRoot>)>,
) {
    for (look, children) in &local_player_query {
        for child in children.iter() {
            if let Ok(mut visual_root_transform) = visual_root_query.get_mut(child) {
                // Yaw belongs to the player's visible facing direction and to
                // movement-relative input. Pitch stays lower in the hierarchy
                // so looking up does not tilt the whole body capsule.
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
}

fn sync_camera_to_player_anchor(
    anchor_query: Query<&GlobalTransform, (With<PlayerCameraAnchor>, With<ChildOf>)>,
    mut camera_query: Single<&mut Transform, (With<PlayerCamera>, Without<PlayerCameraAnchor>)>,
) {
    let Some(anchor_transform) = anchor_query.iter().next() else {
        return;
    };

    camera_query.translation = anchor_transform.translation();
    camera_query.rotation = anchor_transform.rotation();
}
