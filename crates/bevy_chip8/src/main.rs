use bevy::{app::App, DefaultPlugins};
use chip_8::Chip8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Chip8::default())
        .run();
}
