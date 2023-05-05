mod gui;
mod gui_renderer;
mod screen;
mod window;

#[cfg(target_arch = "wasm32")]
mod wasm;

use instant::{Duration, Instant};
use window::Crab8Window;
use winit::{event::Event, event_loop::EventLoop};

use crate::screen::DrawScreen;
pub async fn run() {
    let mut crab8 = crab8::Crab8::default();
    let event_loop = EventLoop::new();
    let mut last_update = Instant::now();

    let test_rom = include_bytes!("../../../roms/1-chip8-logo.ch8");
    crab8.load(test_rom);

    let mut window = {
        let (crab8_width, crab8_height) = crab8.screen.size();
        Crab8Window::new_async(crab8_width as u32, crab8_height as u32, &event_loop)
            .await
            .expect("Failed to build window")
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                crab8.screen.draw_screen(window.pixels.frame_mut());
            }

            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta_time = Instant::now() - last_update;
                last_update = now;

                if delta_time > Duration::from_millis(18) {
                    println!("Long frame detected: {delta_time:?}");
                }

                crab8.execute(crab8::input::Input::default());
            }

            _ => (),
        }

        window.update(&event, control_flow);
    });
}
