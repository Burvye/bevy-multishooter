pub mod player;
pub mod schedule;

use avian3d::prelude::*;
use bevy::prelude::*;

use self::schedule::{SimConfig, SimTick};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let sim_config = SimConfig::default();

        app.insert_resource(Time::<Fixed>::from_hz(sim_config.tps))
            .insert_resource(sim_config)
            .insert_resource(SimTick::default())
            .add_plugins((PhysicsPlugins::default(), player::PlayerPlugin))
            .add_systems(Startup, schedule::configure_tps)
            .add_systems(FixedUpdate, schedule::tick_sim);
    }
}
