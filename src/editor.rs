use bevy::prelude::*;

pub(crate) mod plugin {
    use super::*;

    pub(crate) struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        #[cfg(not(feature = "editor"))]
        fn build(&self, _: &mut App) {}

        #[cfg(feature = "editor")]
        fn build(&self, app: &mut App) {
            use bevy_editor_pls::EditorPlugin;
            app.add_plugin(EditorPlugin);
        }
    }
}
