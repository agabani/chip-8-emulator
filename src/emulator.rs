pub(crate) mod plugin {
    use super::system;

    pub(crate) struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_startup_system(system::audio_setup)
                .add_system(system::drag_and_drop_rom)
                .add_system(system::emulate)
                .add_system(system::keyboard);
        }
    }
}

mod resource {
    use bevy::prelude::*;

    pub(crate) struct Beep(pub(crate) Handle<AudioSource>);
}

mod system {
    use std::io::Read;

    use bevy::prelude::*;

    use crate::chip8::emulator;

    pub(super) fn drag_and_drop_rom(
        mut commands: Commands,
        mut reader: EventReader<FileDragAndDrop>,
    ) {
        for event in reader.iter() {
            match event {
                FileDragAndDrop::DroppedFile { id: _, path_buf } => {
                    let mut rom = Vec::new();
                    let mut file = std::fs::File::open(path_buf).expect("failed to open file");
                    file.read_to_end(&mut rom).expect("failed to read file");

                    let mut emulator = emulator::Emulator::new();
                    emulator.load_rom(&rom).expect("failed to load rom");
                    commands.insert_resource(emulator);
                }
                FileDragAndDrop::HoveredFile { id: _, path_buf: _ }
                | FileDragAndDrop::HoveredFileCancelled { id: _ } => (),
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn emulate(
        time: Res<Time>,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
        beep: Res<super::resource::Beep>,
        mut emulator: ResMut<crate::chip8::emulator::Emulator>,
    ) {
        emulator.emulate(&time.delta());

        if emulator.is_beeping() {
            audio.play(asset_server.get_handle(&beep.0));
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn keyboard(
        keys: Res<Input<KeyCode>>,
        mut emulator: ResMut<crate::chip8::emulator::Emulator>,
    ) {
        use crate::chip8::keypad::Key;

        for (keyboard, keypad) in [
            (KeyCode::X, Key::Key0),
            (KeyCode::Key1, Key::Key1),
            (KeyCode::Key2, Key::Key2),
            (KeyCode::Key3, Key::Key3),
            (KeyCode::Q, Key::Key4),
            (KeyCode::W, Key::Key5),
            (KeyCode::E, Key::Key6),
            (KeyCode::A, Key::Key7),
            (KeyCode::S, Key::Key8),
            (KeyCode::D, Key::Key9),
            (KeyCode::Z, Key::A),
            (KeyCode::C, Key::B),
            (KeyCode::Key4, Key::C),
            (KeyCode::R, Key::D),
            (KeyCode::F, Key::E),
            (KeyCode::V, Key::F),
        ] {
            if keys.just_pressed(keyboard) {
                emulator.key_pressed(keypad);
            }
            if keys.just_released(keyboard) {
                emulator.key_released(keypad);
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn audio_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let music = asset_server.load("beep.ogg");
        commands.insert_resource(super::resource::Beep(music));
    }
}
