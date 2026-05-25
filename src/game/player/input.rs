use bevy::prelude::*;

use super::components::PlayerInput;

/// Clears input that should be reset every tick
pub fn clear_temp_input(mut player_inputs: Query<&mut PlayerInput>) {
    for mut player_input in &mut player_inputs {
        player_input.jump_pressed = false;
    }
}
