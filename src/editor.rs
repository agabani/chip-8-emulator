pub(crate) mod plugin {
    use bevy::prelude::*;

    pub(crate) struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        #[cfg(not(feature = "editor"))]
        fn build(&self, _: &mut App) {}

        #[cfg(feature = "editor")]
        fn build(&self, app: &mut App) {
            use bevy_editor_pls::{
                controls::{self, EditorControls},
                EditorPlugin,
            };

            fn editor_controls() -> EditorControls {
                let mut editor_controls = EditorControls::default_bindings();
                editor_controls.unbind(controls::Action::PlayPauseEditor);

                editor_controls.insert(
                    controls::Action::PlayPauseEditor,
                    controls::Binding {
                        input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::F1)),
                        conditions: vec![controls::BindingCondition::ListeningForText(false)],
                    },
                );

                editor_controls
            }

            app.add_plugin(EditorPlugin)
                .insert_resource(editor_controls());
        }
    }
}
