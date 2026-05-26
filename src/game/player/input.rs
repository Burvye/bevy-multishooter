use super::components as player;
use bevy::prelude::*;

/// Clears input that should be reset every tick
pub fn clear_temp_input(mut player_inputs: Query<&mut player::Input>) {
    for mut player_input in &mut player_inputs {
        player_input.jump_pressed = false;
    }
}
