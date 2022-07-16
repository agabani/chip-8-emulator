use bevy::prelude::*;

use crate::{camera, display, editor, emulator, rom, window};

pub fn run() {
    App::new()
        .insert_resource(window::resource())
        .insert_resource(emulator::Emulator::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(editor::plugin::Plugin)
        .add_startup_system(camera::system::spawn)
        .add_plugin(display::plugin::Plugin)
        .add_system(rom::system::drag_and_drop_loader)
        .add_system(emulator::fetch_decode_execute)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
