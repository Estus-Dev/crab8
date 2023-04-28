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
struct Rom(Vec<u8>);

#[derive(Component)]
/// A marker component for the CHIP-8 screen render
pub struct Screen;

#[derive(Resource)]
/// Stores the number of instructions executed since last frame.
pub struct InstructionsSinceLastFrame(usize);

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
/// The current state of the emulator.
pub enum PlaybackState {
    #[default]
    /// The default state is unloaded, where there is no ROM to execute.
    Unloaded,
    /// When downloading a ROM, the emulator is effectively stopped.
    Downloading,
    /// There is a ROM loaded, execution will begin from the start.
    Stopped,
    /// The emulator is playing at full speed.
    Playing,
    /// The emulator is paused, but emulator state is retained.
    Paused,
    /// The emulator is executing a single instruction.
    StepInstruction,
    /// The emulator is executing a single frame's worth of instructions.
    StepFrame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CRAB-8".to_string(),
                resolution: (1024.0, 512.0 + 48.0).into(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ReqwestPlugin)
        .add_plugin(ui::Plugin)
        .insert_resource(Crab8::default())
        .insert_resource(FixedTime::new_from_secs(TIMESTEP))
        .insert_resource(InstructionsSinceLastFrame(0))
        .add_state::<PlaybackState>()
        .add_system(load_rom.in_set(OnUpdate(PlaybackState::Downloading)))
        .add_startup_system(setup_crab8)
        .add_system(update_crab8.in_schedule(CoreSchedule::FixedUpdate))
        .add_system(reset_crab8.in_schedule(OnExit(PlaybackState::Stopped)))
        .run();
}

fn setup_crab8(mut commands: Commands, mut next_state: ResMut<NextState<PlaybackState>>) {
    if let Ok(url) =
        "https://raw.githubusercontent.com/Timendus/chip8-test-suite/master/bin/1-chip8-logo.ch8"
            .try_into()
    {
        let request = Request::new(Method::GET, url);
        commands.spawn(ReqwestRequest(Some(request)));
        next_state.set(PlaybackState::Downloading);
    }

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.125,
            ..default()
        },
        ..default()
    });
}

fn load_rom(
    mut commands: Commands,
    query: Query<(Entity, &ReqwestBytesResult)>,
    mut next_state: ResMut<NextState<PlaybackState>>,
) {
    if let Ok((entity, response)) = query.get_single() {
        let rom = response
            .as_ref()
            .expect("The network could never fail, right?");

        commands.insert_resource(Rom(rom.to_vec()));
        next_state.set(PlaybackState::Stopped);
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_crab8(
    keyboard: Res<Input<KeyCode>>,
    state: Res<State<PlaybackState>>,
    mut crab8: ResMut<Crab8>,
    mut cycle_count: ResMut<InstructionsSinceLastFrame>,
    mut next_state: ResMut<NextState<PlaybackState>>,
) {
    use PlaybackState::*;

    let input = get_input(keyboard);

    match state.0 {
        StepInstruction if INSTRUCTIONS_PER_TICK - cycle_count.0 == 1 => {
            crab8.execute(input);
            crab8.tick();
            cycle_count.0 = 0;
        }
        StepInstruction => {
            crab8.execute(input);
            cycle_count.0 += 1;
        }
        Playing | StepFrame => {
            for _ in cycle_count.0..INSTRUCTIONS_PER_TICK {
                crab8.execute(input);
            }

            crab8.tick();
            cycle_count.0 = 0;
        }
        _ => (),
    };

    match state.0 {
        StepInstruction | StepFrame => next_state.set(Paused),
        _ => (),
    }
}

fn reset_crab8(mut commands: Commands, rom: Res<Rom>) {
    let mut crab8 = Crab8::default();
    crab8.load(&rom.0);

    commands.remove_resource::<Crab8>();
    commands.insert_resource(crab8);
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
