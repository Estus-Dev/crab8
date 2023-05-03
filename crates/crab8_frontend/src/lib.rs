mod screen;
mod window;

use window::Crab8Window;
use winit::{event::Event, event_loop::EventLoop};

use crate::screen::DrawScreen;
pub fn run() {
    let crab8 = crab8::Crab8::default();
    let event_loop = EventLoop::new();

    let mut window = {
        let (crab8_width, crab8_height) = crab8.screen.size();
        Crab8Window::new(crab8_width as u32, crab8_height as u32, &event_loop)
            .expect("Failed to build window")
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            crab8.screen.draw_screen(window.pixels.frame_mut());
        }

        window.update(&event, control_flow);
    });
}
