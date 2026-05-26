use crate::game::schedule;
use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

use super::components as player;

#[derive(Resource, Debug, Clone)]
pub struct PlayerStats {
    pub accel: Scalar,
    pub speed: Scalar,
    pub jump_speed: Scalar,
    pub gravity: Vector,
    pub fall_speed: Scalar,
    pub horizontal_damping: Scalar,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            accel: 90.0,
            speed: 8.0,
            jump_speed: 7.5,
            gravity: Vector::new(0.0, -30.0, 0.0),
            fall_speed: 40.0,
            horizontal_damping: 10.0,
        }
    }
}

/// checks for groundedness and mutates movement state
pub fn update_grounded_state(
    mut players: Query<
        (
            Entity,
            &player::GroundProbe,
            &GlobalTransform,
            &mut player::MoveState,
            &Collider,
        ),
        With<player::Player>,
    >,
    spatial_query: SpatialQuery,
) {
    for (player, ground_probe, global_transform, mut mov_state, collider) in &mut players {
        let position = global_transform.translation().adjust_precision();
        let rotation = global_transform.rotation().adjust_precision();

        let hit = spatial_query.cast_shape(
            collider,
            position,
            rotation,
            global_transform.down(),
            &ShapeCastConfig::from_max_distance(ground_probe.dist),
            &SpatialQueryFilter::from_excluded_entities([player]),
        );

        // checks if the floor is not more angled than max walkable angle
        mov_state.grounded = hit.is_some_and(|hit| {
            let up = global_transform.up().adjust_precision();
            (rotation * hit.normal1).angle_between(up) <= ground_probe.angle
        });
    }
}

pub fn apply_horizontal_input(
    config: Res<PlayerStats>,
    mut players: Query<
        (&player::Input, &player::LookState, &mut LinearVelocity),
        With<player::Player>,
    >,
    delta_time: Res<schedule::DeltaTime>,
) {
    for (input, look, mut vel) in &mut players {
        // movement interpreted from yaw, the controller gives
        // all the states so we dont depend on stuff like the
        // camera here
        let dir_desired = Quat::from_rotation_y(look.yaw) * local_input(input);

        vel.x += dir_desired.x * config.accel * delta_time.dt;
        vel.z += dir_desired.z * config.accel * delta_time.dt;

        let hoz_speed = Vector2::new(vel.x, vel.z);
        // clamp the horizontal speed
        let hoz_speed = hoz_speed.clamp_length_max(config.speed);
        vel.x = hoz_speed.x;
        vel.z = hoz_speed.y;
    }
}
/// simple function that expands into a local input vector from player input without variables
fn local_input(input: &player::Input) -> Vector3 {
    Vector3::new(input.move_dir.x, 0.0, -input.move_dir.y).clamp_length_max(1.0)
}

pub fn apply_jump(
    config: Res<PlayerStats>,
    mut players: Query<
        (&player::Input, &mut player::MoveState, &mut LinearVelocity),
        With<player::Player>,
    >,
) {
    for (input, mut mov_state, mut vel) in &mut players {
        if input.jump_pressed && mov_state.grounded {
            vel.y = config.jump_speed;
            mov_state.grounded = false;
        }
    }
}

/// Applying gravity with consideration for custom gravity direction
pub fn apply_gravity(
    delta_time: Res<schedule::DeltaTime>,
    config: Res<PlayerStats>,
    mut players: Query<(&player::MoveState, &mut LinearVelocity), With<player::Player>>,
) {
    // TODO: if gravity direction becomes configurable per entity
    // move this logic out of the shared player state and
    // into a component
    // README: the direction of gravity
    // is not automatically assumed to
    // be in the Y direction!
    let grav_dir = config.gravity.normalize_or_zero();

    for (mov_state, mut vel) in &mut players {
        // reset y velocity if player is grounded to prevent accumulation
        if mov_state.grounded && vel.y <= 0.0 {
            vel.y = 0.0;
            continue;
        }

        // the velocity in the direction of gravity
        let vel_on_grav = vel.dot(grav_dir);
        // stop applying gravity if the player is at terminal velocity
        if vel_on_grav > config.fall_speed {
            continue;
        }

        // velocity if we applied gravitational acceleration for d_secs
        let new_vel = vel.0 + config.gravity * delta_time.dt;
        // the gravity direction component of the new velocity
        let new_vel_on_grav = new_vel.dot(grav_dir);

        if new_vel_on_grav < config.fall_speed {
            // not going straight downwards
            vel.0 = new_vel;
        } else {
            // we are at terminal velocity going downwards
            vel.0 = grav_dir * config.fall_speed;
        }
    }
}

pub fn apply_horizontal_damping(
    delta_time: Res<schedule::DeltaTime>,
    config: Res<PlayerStats>,
    mut players: Query<&mut LinearVelocity, With<player::Player>>,
) {
    // something less than 1
    let damping_factor = 1.0 / (1.0 + delta_time.dt * config.horizontal_damping);

    for mut vel in &mut players {
        vel.x *= damping_factor;
        vel.z *= damping_factor;
    }
}

/// apply movement while sliding against walls
pub fn move_player_body(
    time: Res<Time>,
    move_and_slide: MoveAndSlide,
    mut players: Query<
        (Entity, &Collider, &mut Transform, &mut LinearVelocity),
        With<player::Controller>,
    >,
) {
    for (player, collider, mut transform, mut vel) in &mut players {
        // TODO: replace the move and slide config with a default
        // config when tweaking slope behavior, steps, and multiplayer
        // correction
        let result = move_and_slide.move_and_slide(
            collider,
            transform.translation.adjust_precision(),
            transform.rotation.adjust_precision(),
            vel.0,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &SpatialQueryFilter::from_excluded_entities([player]),
            |_| MoveAndSlideHitResponse::Accept,
        );

        transform.translation = result.position.f32();
        vel.0 = result.projected_velocity;
    }
}
