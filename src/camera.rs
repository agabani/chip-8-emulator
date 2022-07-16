pub(crate) mod system {
    use bevy::prelude::*;

    pub(crate) fn spawn(mut commands: Commands) {
        commands
            .spawn_bundle(OrthographicCameraBundle::new_2d())
            .insert(Name::new("camera"));
    }
}
