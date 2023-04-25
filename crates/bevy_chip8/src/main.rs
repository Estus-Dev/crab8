mod screen;

use bevy::prelude::*;
use crab8::{input::Key, Crab8};
use screen::render_framebuffer;

/// CHIP-8 updates timers and display at 60hz
const TIMESTEP: f32 = 1.0 / 60.0;

/// There's not much of a standard for tick rate.
/// This will need to be configurable via UI to make most software work.
const INSTRUCTIONS_PER_TICK: usize = 10;

#[derive(Resource)]
struct Rom(Vec<u8>);

#[derive(Component)]
/// A marker component for the CHIP-8 screen render
struct Screen;

#[tokio::main(flavor = "current_thread")]
async fn main() -> reqwest::Result<()> {
    let rom = reqwest::get(
        "https://raw.githubusercontent.com/Timendus/chip8-test-suite/master/bin/3-corax+.ch8",
    )
    .await?
    .bytes()
    .await?;

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CRAB-8".to_string(),
                resolution: (1024.0, 512.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Crab8::default())
        .insert_resource(FixedTime::new_from_secs(TIMESTEP))
        .insert_resource(Rom(rom.into()))
        .add_startup_system(setup_crab8)
        .add_system(update_crab8.in_schedule(CoreSchedule::FixedUpdate))
        .run();

    Ok(())
}

fn setup_crab8(
    mut commands: Commands,
    rom: Res<Rom>,
    mut images: ResMut<Assets<Image>>,
    mut crab8: ResMut<Crab8>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.125,
            ..default()
        },
        ..default()
    });

    commands
        .spawn(SpriteBundle {
            texture: images.add(render_framebuffer(&crab8.screen)),
            ..default()
        })
        .insert(Screen)
        .insert(Name::new("Screen"));

    crab8.load(&rom.0);
}

fn update_crab8(
    mut commands: Commands,
    query: Query<(Entity, &Handle<Image>, &Screen)>,
    keyboard: Res<Input<KeyCode>>,
    mut images: ResMut<Assets<Image>>,
    mut crab8: ResMut<Crab8>,
) {
    let input = get_input(keyboard);

    for _ in 0..INSTRUCTIONS_PER_TICK {
        crab8.execute(input);
    }

    crab8.tick();

    for (entity, previous_frame, _) in &query {
        commands
            .entity(entity)
            .remove::<Handle<Image>>()
            .insert(images.add(render_framebuffer(&crab8.screen)));

        images.remove(previous_frame);
    }
}

fn get_keybind(key: Key) -> KeyCode {
    use Key::*;

    match key {
        Key0 => KeyCode::X,
        Key1 => KeyCode::Key1,
        Key2 => KeyCode::Key2,
        Key3 => KeyCode::Key3,
        Key4 => KeyCode::Q,
        Key5 => KeyCode::W,
        Key6 => KeyCode::E,
        Key7 => KeyCode::A,
        Key8 => KeyCode::S,
        Key9 => KeyCode::D,
        KeyA => KeyCode::Z,
        KeyB => KeyCode::C,
        KeyC => KeyCode::Key4,
        KeyD => KeyCode::R,
        KeyE => KeyCode::F,
        KeyF => KeyCode::V,
    }
}

fn get_input(keyboard: Res<Input<KeyCode>>) -> crab8::input::Input {
    let mut builder = crab8::input::Input::build();

    for key in 0x0..=0xF {
        let key = Key::try_from(key).expect("A nibble is a valid key");
        let keybind = get_keybind(key);

        builder = builder.set_pressed(key, keyboard.pressed(keybind));
    }

    builder.build()
}
