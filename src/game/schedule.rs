use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct SimConfig {
    /// The server ticks per second
    pub tps: f64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self { tps: 60.0 }
    }
}

#[derive(Resource, Debug, Default)]
pub struct SimTick {
    pub val: u64,
}

pub fn configure_tps(config: Res<SimConfig>) {
    info!(ticks_per_second = config.tps, "tps configured");
}

pub fn tick_sim(mut sim_tick: ResMut<SimTick>) {
    sim_tick.val += 1;
}
