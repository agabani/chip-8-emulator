use bevy::prelude::*;

use crate::{camera, display, editor, window};

pub fn run() {
    App::new()
        .insert_resource(window::resource())
        .add_plugins(DefaultPlugins)
        .add_plugin(editor::plugin::Plugin)
        .add_startup_system(camera::system::spawn)
        .add_startup_system(display::system::spawn)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
