pub(crate) mod component {
    use bevy::prelude::*;

    #[derive(Component)]
    pub(crate) struct Pixel {
        pub(crate) x: u8,
        pub(crate) y: u8,
    }
}

pub(crate) mod plugin {
    use super::system;

    pub(crate) struct Plugin;

    impl bevy::prelude::Plugin for Plugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_startup_system(system::spawn_pixels)
                .add_system(system::recolor_pixels);
        }
    }
}

mod system {
    use bevy::prelude::*;

    use super::component::Pixel;

    pub(super) fn spawn_pixels(mut commands: Commands) {
        fn transform(
            pixel_x: u8,
            pixel_y: u8,
            pixel_size: Vec2,
            pixel_padding_size: f32,
        ) -> Transform {
            Transform::from_xyz(
                f32::from(pixel_x) * (pixel_size.x + pixel_padding_size) + pixel_padding_size / 2.0,
                f32::from(pixel_y) * (pixel_size.y + pixel_padding_size) + pixel_padding_size / 2.0,
                0.0,
            )
        }

        let display_size = Vec2::new(1280.0, 640.0);
        let pixels_x: u8 = 64;
        let pixels_y: u8 = 32;
        let pixel_padding_size: f32 = 4.0;

        let pixel_size = Vec2::new(
            display_size.x / f32::from(pixels_x) - pixel_padding_size,
            display_size.y / f32::from(pixels_y) - pixel_padding_size,
        );

        commands
            .spawn()
            .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
                -display_size.x / 2.0 + pixel_size.x - pixel_padding_size * 2.0,
                -display_size.y / 2.0 + pixel_size.y - pixel_padding_size * 2.0,
                0.0,
            )))
            .insert(Name::new("display"))
            .with_children(|display| {
                for pixel_y in 0..pixels_y {
                    for pixel_x in 0..pixels_x {
                        display
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::Rgba {
                                        red: 255.0,
                                        green: 255.0,
                                        blue: 255.0,
                                        alpha: 1.0,
                                    },
                                    custom_size: Some(pixel_size),
                                    ..Default::default()
                                },
                                transform: transform(
                                    pixel_x,
                                    pixel_y,
                                    pixel_size,
                                    pixel_padding_size,
                                ),
                                ..Default::default()
                            })
                            .insert(Name::new(format!(
                                "pixel x:{:0>2} y:{:0>2}",
                                pixel_x, pixel_y
                            )))
                            .insert(Pixel {
                                x: pixel_x,
                                y: pixel_y,
                            });
                    }
                }
            });
    }

    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn recolor_pixels(
        emulator: Res<crate::chip8::Emulator>,
        mut query: Query<(&Pixel, &mut Sprite)>,
    ) {
        for (pixel, mut sprite) in query.iter_mut() {
            if emulator.is_pixel_on(pixel.x, pixel.y) {
                sprite.color = Color::Rgba {
                    red: 255.0,
                    green: 255.0,
                    blue: 255.0,
                    alpha: 1.0,
                };
            } else {
                sprite.color = Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                };
            }
        }
    }
}
