mod gui;
mod input;
mod screen;
mod window;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use instant::{Duration, Instant};
use reqwest::{Client, Method, Request};
use window::Crab8Window;
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
};

use crate::screen::DrawScreen;
pub async fn run() {
    let mut crab8 = crab8::Crab8::default();
    let event_loop = EventLoop::new();
    let mut last_update = Instant::now();
    let url =
        "https://raw.githubusercontent.com/Timendus/chip8-test-suite/master/bin/1-chip8-logo.ch8"
            .try_into()
            .expect("It's a valid URL");

    let test_rom = Client::new()
        .execute(Request::new(Method::GET, url))
        .await
        .expect("The network could never fail, right?")
        .bytes()
        .await
        .expect("I should look up how getting a successful response's bytes could fail.");

    crab8.load(&test_rom);

    let mut window = {
        let (crab8_width, crab8_height) = crab8.screen.size();
        Crab8Window::new_async(crab8_width as u32, crab8_height as u32, &event_loop)
            .await
            .expect("Failed to build window")
    };

    event_loop.run(move |event, _, control_flow| {
        match &event {
            Event::RedrawRequested(_) => {
                crab8.screen.draw_screen(window.pixels.frame_mut());
            }

            // Clippy insists this is the idiomatic way to handle this event...
            // All I'm doing here is getting the keycode and current state of this event
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                input::handle_input(*keycode, *state, &mut crab8);
            }

            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = Instant::now() - last_update;
                last_update = now;

                if delta_time > Duration::from_millis(18) {
                    log::warn!("Long frame detected: {delta_time:?}");
                }

                crab8.execute();
            }

            _ => (),
        }

        window.update(&event, control_flow, &mut crab8);
    });
}
