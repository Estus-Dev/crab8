mod screen;
mod window;

use window::Crab8Window;
use winit::{event::Event, event_loop::ControlFlow};
use winit_input_helper::WinitInputHelper;

use crate::screen::DrawScreen;
pub fn run() {
    let crab8 = crab8::Crab8::default();

    let mut window = {
        let (crab8_width, crab8_height) = crab8.screen.size();
        Crab8Window::new(crab8_width as u32, crab8_height as u32).expect("Failed to build window")
    };

    let mut input = WinitInputHelper::new();

    window.event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = input.window_resized() {
                if window
                    .pixels
                    .resize_surface(size.width, size.height)
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
        }

        // I also plan to expand this match statement clippy
        #[allow(clippy::single_match)]
        match event {
            Event::RedrawRequested(_) => {
                crab8.screen.draw_screen(window.pixels.frame_mut());

                let render_result = window
                    .pixels
                    .render_with(|encoder, render_target, context| {
                        context.scaling_renderer.render(encoder, render_target);

                        Ok(())
                    });

                if let Err(_err) = render_result {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
