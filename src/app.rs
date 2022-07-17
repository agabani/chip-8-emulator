use bevy::prelude::*;

use crate::{camera, chip8, display, editor, emulator, window};

pub fn run() {
    App::new()
        .insert_resource(window::resource())
        .insert_resource(chip8::Emulator::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(editor::plugin::Plugin)
        .add_startup_system(camera::system::spawn)
        .add_plugin(display::plugin::Plugin)
        .add_system(emulator::system::drag_and_drop_rom)
        .add_system(emulator::system::emulate)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
