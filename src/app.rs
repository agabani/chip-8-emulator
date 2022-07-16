use bevy::prelude::*;

use crate::window;

pub fn run() {
    let mut app = App::new();

    app.insert_resource(window::resource())
        .add_plugins(DefaultPlugins);

    add_editor(&mut app);

    app.add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

#[cfg(not(feature = "editor"))]
fn add_editor(_: &mut App) {}

#[cfg(feature = "editor")]
fn add_editor(app: &mut App) {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugin(EditorPlugin);
}
