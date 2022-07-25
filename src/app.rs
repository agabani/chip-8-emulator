use bevy::prelude::*;

use crate::{camera, chip8, display, emulator, window};

pub fn run() {
    let mut app = App::new();

    app.insert_resource(window::resource())
        .insert_resource(chip8::Emulator::new())
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera::system::spawn)
        .add_plugin(display::plugin::Plugin)
        .add_plugin(emulator::plugin::Plugin)
        .add_system(bevy::input::system::exit_on_esc_system);

    #[cfg(feature = "editor")]
    {
        use crate::editor;
        app.add_plugin(editor::plugin::Plugin);
    }

    app.run();
}
