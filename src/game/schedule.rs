use avian3d::math::*;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct SimConfig {
    /// The server ticks per second
    pub tps: f64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self { tps: 20.0 }
    }
}

#[derive(Resource, Debug, Default)]
pub struct SimTick {
    pub val: u64,
}

#[derive(Resource, Debug, Clone)]
pub struct DeltaTime {
    pub dt: Scalar,
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self { dt: 0.0 }
    }
}

pub fn configure_tps(config: Res<SimConfig>) {
    info!(ticks_per_second = config.tps, "tps configured");
}

pub fn tick_sim(mut sim_tick: ResMut<SimTick>) {
    sim_tick.val += 1;
}

pub fn update_delta_time(time: Res<Time>, mut delta_time: ResMut<DeltaTime>) {
    delta_time.dt = time.delta_secs_f64().adjust_precision()
}
