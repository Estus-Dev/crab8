mod screen;

use bevy::prelude::*;
use chip_8::{input::Input, Chip8};
use screen::render_framebuffer;

/// CHIP-8 updates timers and display at 60hz
const TIMESTEP: f32 = 1.0 / 60.0;

/// There's not much of a standard for tick rate.
/// This will need to be configurable via UI to make most software work.
const INSTRUCTIONS_PER_TICK: usize = 10;

#[derive(Component)]
/// A marker component for the CHIP-8 screen render
struct Screen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CHIP-8".to_string(),
                resolution: (1024.0, 512.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Chip8::default())
        .insert_resource(FixedTime::new_from_secs(TIMESTEP))
        .add_startup_system(setup_chip8)
        .add_system(update_chip8.in_schedule(CoreSchedule::FixedUpdate))
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

    commands
        .spawn(SpriteBundle {
            texture: images.add(render_framebuffer(&chip8.screen)),
            ..default()
        })
        .insert(Screen)
        .insert(Name::new("Screen"));
}

fn update_chip8(
    mut commands: Commands,
    query: Query<(Entity, &Handle<Image>, &Screen)>,
    mut images: ResMut<Assets<Image>>,
    mut chip8: ResMut<Chip8>,
) {
    let input = Input::default();

    for _ in 0..INSTRUCTIONS_PER_TICK {
        chip8.execute(input);
    }

    chip8.tick();

    for (entity, previous_frame, _) in &query {
        commands
            .entity(entity)
            .remove::<SpriteBundle>()
            .insert(SpriteBundle {
                texture: images.add(render_framebuffer(&chip8.screen)),
                ..default()
            });

        images.remove(previous_frame);
    }
}
