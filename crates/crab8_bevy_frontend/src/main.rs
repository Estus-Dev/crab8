mod screen;
mod ui;

use bevy::prelude::*;
use bevy_mod_reqwest::{ReqwestBytesResult, ReqwestPlugin, ReqwestRequest};
use crab8::{input::Key, Crab8};
use reqwest::{Method, Request};

/// CHIP-8 updates timers and display at 60hz
const TIMESTEP: f32 = 1.0 / 60.0;

/// There's not much of a standard for tick rate.
/// This will need to be configurable via UI to make most software work.
const INSTRUCTIONS_PER_TICK: usize = 10;

#[derive(Resource)]
// Temporary resource to flag whether there is a ROM loaded.  This should be a Bevy state.
struct Running(bool);

#[derive(Resource)]
struct Rom(Vec<u8>);

#[derive(Component)]
/// A marker component for the CHIP-8 screen render
pub struct Screen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CRAB-8".to_string(),
                resolution: (1024.0, 512.0).into(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ReqwestPlugin)
        .add_plugin(ui::Plugin)
        .insert_resource(Crab8::default())
        .insert_resource(FixedTime::new_from_secs(TIMESTEP))
        .insert_resource(Running(false))
        .add_system(load_rom)
        .add_startup_system(setup_crab8)
        .add_system(update_crab8.in_schedule(CoreSchedule::FixedUpdate))
        .run();
}

fn setup_crab8(mut commands: Commands) {
    if let Ok(url) =
        "https://raw.githubusercontent.com/Timendus/chip8-test-suite/master/bin/1-chip8-logo.ch8"
            .try_into()
    {
        let request = Request::new(Method::GET, url);
        commands.spawn(ReqwestRequest(Some(request)));
    }

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.125,
            ..default()
        },
        ..default()
    });
}

// TODO: This should transition bevy states
fn load_rom(
    mut commands: Commands,
    query: Query<(Entity, &ReqwestBytesResult)>,
    mut crab8: ResMut<Crab8>,
    mut running: ResMut<Running>,
) {
    for (entity, response) in query.iter() {
        let rom = response
            .as_ref()
            .expect("The network could never fail, right?");
        crab8.load(rom);
        running.0 = true;

        commands.entity(entity).despawn_recursive();
    }
}

fn update_crab8(keyboard: Res<Input<KeyCode>>, running: Res<Running>, mut crab8: ResMut<Crab8>) {
    if !running.0 {
        return;
    }

    let input = get_input(keyboard);

    for _ in 0..INSTRUCTIONS_PER_TICK {
        crab8.execute(input);
    }

    crab8.tick();
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
