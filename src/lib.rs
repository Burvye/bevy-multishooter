pub mod app;
pub mod client;
pub mod game;
pub mod net;
pub mod server;

use bevy::prelude::*;

pub fn run() -> AppExit {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, app::ProjectAppPlugin));
    app.run()
}
