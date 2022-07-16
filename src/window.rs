use bevy::prelude::*;

pub(crate) fn resource() -> WindowDescriptor {
    WindowDescriptor {
        title: "CHIP-8 Emulator".into(),
        ..Default::default()
    }
}
