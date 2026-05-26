use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::game::player::components::{
    LCamAnchor, LocalPlayer, LookState, MoveState, Player, VisRoot,
};

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct ModelRoot;

#[derive(Component, Default)]
pub struct PresentationBound;

#[derive(Component, Default)]
pub struct CameraBound;

#[derive(Component, Debug, Clone, Copy)]
pub struct VisualState {
    pub grounded: bool,
    pub moving: bool,
    /// horizontal movement speed
    pub speed: f32,
    /// jumping velocity impulse
    pub jump_speed: f32,
}

impl Default for VisualState {
    fn default() -> Self {
        Self {
            grounded: false,
            moving: false,
            speed: 0.0,
            jump_speed: 0.0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct ModelConfig {
    pub scene_path: &'static str,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            scene_path: "models/person.glb#Scene0",
            translation: Vec3::new(0.0, -0.875, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ModelConfig::default())
            .add_systems(Startup, spawn_local_camera)
            .add_systems(
                Update,
                (
                    bind_player_presentation,
                    bind_local_camera_to_anchor,
                    update_visual_state,
                    sync_local_player_visuals,
                )
                    .chain(),
            );
    }
}

// TODO: replace local camera with an explicit camera once visible
// self, remote player bodies are defined
fn spawn_local_camera(mut cmds: Commands) {
    cmds.spawn((PlayerCamera, Camera3d::default(), Transform::IDENTITY));
}

fn bind_local_camera_to_anchor(
    mut cmds: Commands,
    local_anchor: Single<Entity, (With<LCamAnchor>, Without<CameraBound>)>,
    camera: Single<Entity, (With<PlayerCamera>, Without<ChildOf>)>,
) {
    cmds.entity(*local_anchor).insert(CameraBound);
    cmds.entity(*local_anchor).add_child(*camera);
}

fn bind_player_presentation(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    model_config: Res<ModelConfig>,
    player_query: Query<(Entity, Option<&LocalPlayer>), With<Player>>,
    mut vis_roots: Query<(Entity, &ChildOf), (With<VisRoot>, Without<PresentationBound>)>,
) {
    for (vis_root, parent) in &mut vis_roots {
        let Ok((player, local_player)) = player_query.get(parent.parent()) else {
            continue;
        };

        cmds.entity(player).insert(VisualState::default());

        let scene = assets.load(model_config.scene_path);

        // TODO: tweak the transform to be correct in game
        // The config is here so animation and model modification
        // doesn't change the simulation logic
        let model_transform = Transform {
            translation: model_config.translation,
            rotation: model_config.rotation,
            scale: model_config.scale,
        };

        // TODO: eventually get a visible self
        let local_visibility = if local_player.is_some() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };

        cmds.entity(vis_root)
            .insert(PresentationBound)
            .with_children(|vis_root| {
                vis_root
                    .spawn((
                        Name::new("PlayerModelRoot"),
                        ModelRoot,
                        local_visibility,
                        model_transform,
                    ))
                    .with_children(|model_root| {
                        model_root.spawn((Name::new("PlayerModelScene"), SceneRoot(scene)));
                    });
            });
    }
}

fn update_visual_state(
    mut cmds: Commands,
    player_query: Query<(Entity, &MoveState, &LinearVelocity, Option<&VisualState>), With<Player>>,
) {
    for (player, move_state, velocity, visual_state) in &player_query {
        let horizontal_speed = Vec2::new(velocity.x, velocity.z).length();
        let next_state = VisualState {
            grounded: move_state.grounded,
            moving: horizontal_speed > 0.05,
            speed: horizontal_speed,
            jump_speed: velocity.y,
        };

        if let Some(current_state) = visual_state {
            if current_state.grounded == next_state.grounded
                && current_state.moving == next_state.moving
                && (current_state.speed - next_state.speed).abs() < 0.001
                && (current_state.jump_speed - next_state.jump_speed).abs() < 0.001
            {
                continue;
            }
        }

        cmds.entity(player).insert(next_state);
    }
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
            // yaw affects what is visible and movement input
            visual_root_transform.rotation = Quat::from_rotation_y(look.yaw);
            // TODO: make pitch rotate the head/arms up and down
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
