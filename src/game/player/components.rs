use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component, Debug, Clone, Copy)]
pub struct Body {
    pub radius: f32,
    pub height: f32,
    pub eye_height: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            radius: 0.75,
            height: 2.0,
            eye_height: 1.75,
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Input {
    /// directional input via WASD or arrow keys
    pub move_dir: Vec2,
    /// jump input
    pub jump_pressed: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct GroundProbe {
    /// distance from ground to be grounded
    pub dist: f32,
    /// max angle for groundedness
    pub angle: f32,
}

impl Default for GroundProbe {
    fn default() -> Self {
        Self {
            dist: 0.15,
            angle: 60.0_f32.to_radians(),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct MoveState {
    pub grounded: bool,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LookState {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for LookState {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(Component)]
pub struct VisRoot;

#[derive(Component)]
pub struct CamAnchor;

#[derive(Component)]
pub struct LCamAnchor;

#[derive(Component, Default)]
#[require(
    RigidBody::Kinematic,
    CustomPositionIntegration,
    // keep SpeculativeMargin as zero for deterministic
    // movement in the player. SpeculativeMargin predicts
    // collisions before they happen based on velocity and
    // this margin.
    // FUTURE: Change this if the movement controller changes
    SpeculativeMargin(0.0),
    LinearVelocity::ZERO
)]
pub struct Controller;
