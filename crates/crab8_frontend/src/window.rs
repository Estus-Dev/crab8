use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 512.0;

pub struct Crab8Window {
    input: WinitInputHelper,
    // pub pixels: Pixels,
}

impl Crab8Window {
    pub fn new(
        crab8_width: u32,
        crab8_height: u32,
        event_loop: &EventLoop<()>,
    ) -> Result<Crab8Window, pixels::Error> {
        let window_size = LogicalSize::new(WIDTH, HEIGHT);

        let winit = WindowBuilder::new()
            .with_title("CRAB-8")
            .with_min_inner_size(window_size)
            .with_inner_size(window_size)
            .build(event_loop)
            .expect("Failed to build winit window");

        let surface = SurfaceTexture::new(crab8_width, crab8_height, &winit);

        // Broken until I refactor to use Pixels::new_async
        // let mut pixels = Pixels::new(crab8_width, crab8_height, surface)?;
        // let window_inner_size = winit.inner_size();
        // pixels.resize_surface(window_inner_size.width, window_inner_size.height)?;

        Ok(Crab8Window {
            // pixels,
            input: WinitInputHelper::new(),
        })
    }

    pub fn update(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        if self.input.update(event) {
            if self.input.close_requested() || self.input.destroyed() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = self.input.window_resized() {
                // if self.pixels.resize_surface(size.width, size.height).is_err() {
                //     *control_flow = ControlFlow::Exit;
                // }
            }
        }

        // if let Event::RedrawRequested(_) = event {
        //     let render_result = self.pixels.render_with(|encoder, render_target, context| {
        //         context.scaling_renderer.render(encoder, render_target);

        //         Ok(())
        //     });

        //     if let Err(_err) = render_result {
        //         *control_flow = ControlFlow::Exit;
        //     }
        // }
    }
}
