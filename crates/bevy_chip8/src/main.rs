mod screen;

use bevy::prelude::*;
use chip_8::Chip8;
use screen::render_framebuffer;

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

fn setup_chip8(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut chip8: ResMut<Chip8>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.125,
            ..default()
        },
        ..default()
    });

    // Set a character in VX
    chip8.exec(0x600F);
    // Load the address of the sprite for the character from VX into I
    chip8.exec(0xF029);
    // Set X position to draw on the screen
    chip8.exec(0x6010);
    // Set Y position to draw on the screen
    chip8.exec(0x6108);
    // Draw the sprite at I to X, Y
    chip8.exec(0xD015);

    commands.spawn(SpriteBundle {
        texture: images.add(render_framebuffer(&chip8.screen)),
        ..default()
    });
}
