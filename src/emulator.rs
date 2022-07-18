pub(crate) mod component {
    use bevy::prelude::*;

    #[derive(Component)]
    pub(crate) struct Content(pub(crate) Vec<u8>);

    #[derive(Component)]
    pub(crate) struct Path(pub(crate) String);

    impl Content {
        pub(crate) fn new(binary: Vec<u8>) -> Content {
            Content(binary)
        }
    }

    impl Path {
        pub(crate) fn new(path: String) -> Path {
            Path(path)
        }
    }
}

pub(crate) mod plugin {
    use super::system;

    pub(crate) struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_system(system::drag_and_drop_rom)
                .add_system(system::emulate);
        }
    }
}

mod system {
    use std::io::Read;

    use bevy::prelude::*;

    use super::component;

    pub(super) fn drag_and_drop_rom(
        mut commands: Commands,
        mut reader: EventReader<FileDragAndDrop>,
        mut emulator: ResMut<crate::chip8::Emulator>,
    ) {
        for event in reader.iter() {
            match event {
                FileDragAndDrop::DroppedFile { id: _, path_buf } => {
                    let mut rom = Vec::new();
                    let mut file = std::fs::File::open(path_buf).expect("failed to open file");
                    file.read_to_end(&mut rom).expect("failed to read file");

                    emulator.load_rom(&rom).expect("failed to load rom");

                    commands
                        .spawn()
                        .insert(Name::new("rom"))
                        .insert(component::Path::new(path_buf.to_str().unwrap().to_string()))
                        .insert(component::Content::new(rom));
                }
                FileDragAndDrop::HoveredFile { id: _, path_buf: _ }
                | FileDragAndDrop::HoveredFileCancelled { id: _ } => (),
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn emulate(time: Res<Time>, mut emulator: ResMut<crate::chip8::Emulator>) {
        emulator.emulate(&time.delta());
    }
}
