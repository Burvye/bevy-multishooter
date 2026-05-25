use bevy::prelude::*;

use crate::{client::ClientPlugin, game::GamePlugin, net::NetPlugin, server::ServerPlugin};

pub struct ProjectAppPlugin;

impl Plugin for ProjectAppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GamePlugin, NetPlugin, ClientPlugin, ServerPlugin));
    }
}
