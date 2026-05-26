pub mod player;
pub mod schedule;

use avian3d::prelude::*;
use bevy::prelude::*;

use self::schedule::{DeltaTime, SimConfig, SimTick};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let sim_config = SimConfig::default();

        app.insert_resource(Time::<Fixed>::from_hz(sim_config.tps))
            .insert_resource(sim_config)
            .insert_resource(SimTick::default())
            .insert_resource(DeltaTime::default())
            .add_plugins((PhysicsPlugins::default(), player::PlayerPlugin))
            .add_systems(Startup, schedule::configure_tps)
            .add_systems(FixedUpdate, (schedule::update_delta_time, schedule::tick_sim).chain());
    }
}
