use bevy::prelude::*;
use chip_8::Chip8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CHIP-8".to_string(),
                resolution: (1024.0, 512.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Chip8::default())
        .add_startup_system(setup_chip8)
        .run();
}

fn setup_chip8(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.125,
            ..default()
        },
        ..default()
    });
}
