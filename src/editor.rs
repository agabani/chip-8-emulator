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

    #[derive(Default)]
    pub struct EmulatorWindowState {
        pub follow_program_counter: bool,
    }

    impl EditorWindow for EmulatorWindow {
        type State = EmulatorWindowState;

        const NAME: &'static str = "Emulator";

        const DEFAULT_SIZE: (f32, f32) = (480.0, 240.0);

        fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
            let state = cx.state_mut::<EmulatorWindow>().unwrap();

            let emulator = world.get_resource::<crate::chip8::Emulator>().unwrap();

            let debug = emulator.get_debug();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::CollapsingHeader::new("Register").show(ui, |ui| {
                        egui::Grid::new("register").striped(false).show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.separator();
                                ui.label("Index");
                                ui.horizontal(|ui| {
                                    ui.label("I:");
                                    ui.label(format!("{:016X}", debug.register_i));
                                });
                            });

                            ui.vertical(|ui| {
                                ui.separator();
                                ui.label("Program Counter");
                                ui.horizontal(|ui| {
                                    ui.label("PC:");
                                    ui.label(format!("{:016X}", debug.register_program_counter));
                                });
                            });

                            ui.end_row();

                            ui.vertical(|ui| {
                                ui.separator();
                                ui.label("V");
                                egui::Grid::new("grid_vx").striped(true).show(ui, |ui| {
                                    ui.label("x");
                                    ui.label("Vx");
                                    ui.end_row();

                                    for (x, vx) in debug.register_v.iter().enumerate() {
                                        ui.label(format!("{:01X}", x));
                                        ui.label(format!("{:02X}", vx));
                                        ui.end_row();
                                    }
                                });
                            });

                            ui.vertical(|ui| {
                                ui.separator();
                                ui.label("Stack");
                                egui::Grid::new("grid stack").striped(true).show(ui, |ui| {
                                    ui.label("x");
                                    ui.label("PC");
                                    ui.end_row();

                                    for (x, pc) in debug.register_stack.iter().enumerate() {
                                        ui.label(format!("{:01X}:", x));
                                        ui.label(format!("{:016X}", pc));
                                        ui.end_row();
                                    }
                                });
                            });

                            ui.end_row();
                        });
                    });

                    egui::CollapsingHeader::new("Memory").show(ui, |ui| {
                        let mut scroll_top = false;
                        let mut scroll_bottom = false;
                        let mut scroll_program_counter = false;

                        ui.horizontal(|ui| {
                            scroll_top |= ui.button("Scroll to top").clicked();
                            scroll_bottom |= ui.button("Scroll to bottom").clicked();
                            scroll_program_counter |=
                                ui.button("Scroll to program counter").clicked();
                            ui.checkbox(
                                &mut state.follow_program_counter,
                                "Follow program counter",
                            );
                        });

                        ui.separator();

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                if scroll_top {
                                    ui.scroll_to_cursor(Some(egui::Align::TOP));
                                }

                                egui::Grid::new("memory_ram").striped(true).show(ui, |ui| {
                                    ui.label("Address");
                                    for i in 0..=0xF {
                                        ui.label(format!("{:1X}", i));
                                    }
                                    ui.end_row();

                                    for (i, bytes) in debug.memory_ram.chunks(16).enumerate() {
                                        ui.label(format!("{:08X}", i * 0x10));
                                        for (j, byte) in bytes.iter().enumerate() {
                                            if i as u16 * 0x10 + j as u16
                                                == debug.register_program_counter
                                            {
                                                let response = ui.colored_label(
                                                    egui::Color32::YELLOW,
                                                    format!("{:02X}", byte),
                                                );
                                                if scroll_program_counter
                                                    || state.follow_program_counter
                                                {
                                                    response.scroll_to_me(Some(egui::Align::Center))
                                                }
                                            } else {
                                                ui.label(format!("{:02X}", byte));
                                            }
                                        }
                                        ui.end_row();
                                    }
                                });

                                if scroll_bottom {
                                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                                }
                            });
                    });

                    if ui.button("-").clicked() {
                        println!("hi")
                        // *counter -= 1;
                    }
                    if ui.button("+").clicked() {
                        println!("ho")
                        // *counter += 1;
                    }
                });
        }

        fn app_setup(app: &mut App) {
            let _ = app;
        }
    }
}
