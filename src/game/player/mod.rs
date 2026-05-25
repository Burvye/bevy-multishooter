pub mod components;
pub mod input;
pub mod movement;
pub mod spawn;

use bevy::prelude::*;

use self::movement::PlayerStats;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerStats::default())
            .add_systems(Startup, (spawn::spawn_test_player, spawn::spawn_test_world))
            .add_systems(
                FixedUpdate,
                (
                    movement::update_grounded_state,
                    movement::apply_horizontal_input,
                    movement::apply_horizontal_damping,
                    movement::apply_jump,
                    movement::apply_gravity,
                    movement::move_player_body,
                    input::clear_temp_input,
                )
                    .chain(),
            );
    }
}
