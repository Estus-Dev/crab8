use crab8::Crab8;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::{gui::renderer::GuiRenderer, gui::Gui};

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 512.0;

pub struct Crab8Window {
    pub gui: Gui,
    gui_renderer: GuiRenderer,
    input: WinitInputHelper,
    pub pixels: Pixels,
    winit: Window,
}

impl Crab8Window {
    pub async fn new_async(
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

        #[cfg(target_arch = "wasm32")]
        crate::wasm::insert_canvas(&winit);

        let surface = SurfaceTexture::new(crab8_width, crab8_height, &winit);

        let mut pixels = Pixels::new_async(crab8_width, crab8_height, surface).await?;
        let window_inner_size = winit.inner_size();
        pixels.resize_surface(window_inner_size.width, window_inner_size.height)?;

        let gui = Gui::new();
        let gui_renderer = GuiRenderer::new(
            event_loop,
            WIDTH as u32,
            HEIGHT as u32,
            winit.scale_factor() as f32,
            &pixels,
        );

        Ok(Crab8Window {
            gui,
            gui_renderer,
            pixels,
            winit,
            input: WinitInputHelper::new(),
        })
    }

    pub fn update(&mut self, event: &Event<()>, control_flow: &mut ControlFlow, crab8: &mut Crab8) {
        if self.input.update(event) {
            if self.input.close_requested() || self.input.destroyed() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = self.input.window_resized() {
                if self.pixels.resize_surface(size.width, size.height).is_err() {
                    *control_flow = ControlFlow::Exit;
                }

                self.gui_renderer.resize(size.width, size.height);
            }

            if let Some(scale_factor) = self.input.scale_factor() {
                self.gui_renderer.scale_factor(scale_factor);
            }

            self.winit.request_redraw();
        }

        match event {
            Event::RedrawRequested(_) => {
                self.gui_renderer.prepare(&mut self.gui, &self.winit, crab8);

                let render_result = self.pixels.render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);

                    self.gui_renderer.render(encoder, render_target, context);

                    Ok(())
                });

                if let Err(_err) = render_result {
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::WindowEvent { event, .. } => {
                self.gui_renderer.handle_event(event);
            }

            _ => (),
        }
    }
}
