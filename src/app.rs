use bevy::prelude::*;

pub fn run() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    add_editor(&mut app);
    app.run();
}

#[cfg(not(feature = "editor"))]
fn add_editor(_: &mut App) {}

#[cfg(feature = "editor")]
fn add_editor(app: &mut App) {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugin(EditorPlugin);
}
