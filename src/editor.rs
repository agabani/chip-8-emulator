pub(crate) mod plugin {
    use bevy::prelude::*;
    use bevy_editor_pls::AddEditorWindow;

    pub(crate) struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
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
                .insert_resource(editor_controls())
                .add_editor_window::<super::window::EmulatorWindow>();
        }
    }
}

mod window {
    use bevy::prelude::*;
    use bevy_editor_pls::{
        editor_window::{EditorWindow, EditorWindowContext},
        egui,
    };

    pub struct EmulatorWindow;

    impl EditorWindow for EmulatorWindow {
        type State = ();

        const NAME: &'static str = "Emulator";

        const DEFAULT_SIZE: (f32, f32) = (480.0, 240.0);

        fn ui(world: &mut World, _cx: EditorWindowContext, ui: &mut egui::Ui) {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |_ui| {});
        }
    }
}
