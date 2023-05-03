use crate::screen::DrawScreen;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WINDOW_WIDTH: f64 = 1024.0;
const WINDOW_HEIGHT: f64 = 512.0;

pub fn build_window() -> Result<(), pixels::Error> {
    let crab8 = crab8::Crab8::default();
    let (crab8_width, crab8_height) = crab8.screen.size();
    let (crab8_width, crab8_height) = (crab8_width as u32, crab8_height as u32);

    let event_loop = EventLoop::new();
    let window_size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let winit_window = WindowBuilder::new()
        .with_title("CRAB-8")
        .with_min_inner_size(window_size)
        .with_inner_size(window_size)
        .build(&event_loop)
        .expect("Failed to build winit window");

    let surface_texture = SurfaceTexture::new(crab8_width, crab8_height, &winit_window);
    let mut pixels = Pixels::new(crab8_width, crab8_height, surface_texture)?;
    let window_inner_size = winit_window.inner_size();
    pixels.resize_surface(window_inner_size.width, window_inner_size.height)?;

    let mut input = WinitInputHelper::new();

    let mut egui_state = egui_winit::State::new(&event_loop);
    egui_state.set_pixels_per_point(winit_window.scale_factor() as f32);

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = input.window_resized() {
                if pixels.resize_surface(size.width, size.height).is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
        }

        // I also plan to expand this match statement clippy
        #[allow(clippy::single_match)]
        match event {
            Event::RedrawRequested(_) => {
                crab8.screen.draw_screen(pixels.frame_mut());

                let render_result = pixels.render_with(|encoder, render_target, context| {
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
