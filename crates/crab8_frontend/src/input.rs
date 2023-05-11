use crab8::{input::Key, Crab8};
use winit::event::{ElementState, VirtualKeyCode};

pub fn handle_input(keycode: VirtualKeyCode, state: ElementState, crab8: &mut Crab8) {
    if let Some(key) = get_keybind(keycode) {
        match state {
            ElementState::Pressed => crab8.next_input.set_pressed(key),
            ElementState::Released => crab8.next_input.set_released(key),
        };
    };
}

/// | VIP Layout | Modern Layout|
/// |------------|--------------|
/// | 1 2 3 C    | 1 2 3 4      |
/// | 4 5 6 D    | Q W E R      |
/// | 7 8 9 E    | A S D F      |
/// | A 0 B F    | Z X C V      |
fn get_keybind(keycode: VirtualKeyCode) -> Option<Key> {
    match keycode {
        VirtualKeyCode::Key1 => Some(Key::Key1),
        VirtualKeyCode::Key2 => Some(Key::Key2),
        VirtualKeyCode::Key3 => Some(Key::Key3),
        VirtualKeyCode::Key4 => Some(Key::KeyC),

        VirtualKeyCode::Q => Some(Key::Key4),
        VirtualKeyCode::W => Some(Key::Key5),
        VirtualKeyCode::E => Some(Key::Key6),
        VirtualKeyCode::R => Some(Key::KeyD),

        VirtualKeyCode::A => Some(Key::Key7),
        VirtualKeyCode::S => Some(Key::Key8),
        VirtualKeyCode::D => Some(Key::Key9),
        VirtualKeyCode::F => Some(Key::KeyE),

        VirtualKeyCode::Z => Some(Key::KeyA),
        VirtualKeyCode::X => Some(Key::Key0),
        VirtualKeyCode::C => Some(Key::KeyB),
        VirtualKeyCode::V => Some(Key::KeyF),

        _ => None,
    }
}

// fn get_input(keyboard: Res<Input<KeyCode>>) -> crab8::input::Input {
//     let mut builder = crab8::input::Input::build();

//     for key in 0x0..=0xF {
//         let key = Key::try_from(key).expect("A nibble is a valid key");
//         let keybind = get_keybind(key);

//         builder = builder.set_pressed(key, keyboard.pressed(keybind));
//     }

//     builder.build()
// }
