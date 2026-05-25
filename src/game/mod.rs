pub mod schedule;

use avian3d::prelude::*;
use bevy::prelude::*;

use self::schedule::{SimConfig, SimTick};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let simulation_config = SimConfig::default();

        app.insert_resource(Time::<Fixed>::from_hz(simulation_config.tps))
            .insert_resource(simulation_config)
            .insert_resource(SimTick::default())
            .add_plugins(PhysicsPlugins::default())
            .add_systems(Startup, schedule::configure_tps)
            .add_systems(FixedUpdate, schedule::tick_sim);
    }
}
