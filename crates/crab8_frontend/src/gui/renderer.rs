use crate::gui::Gui;
use crab8::Crab8;
use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::{
    renderer::ScreenDescriptor,
    wgpu::{
        CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
        TextureView,
    },
    Renderer,
};
use egui_winit::State;
use pixels::{Pixels, PixelsContext};
use winit::{event::WindowEvent, event_loop::EventLoopWindowTarget};

pub struct GuiRenderer {
    context: Context,
    state: State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    textures: TexturesDelta,
    paint_jobs: Vec<ClippedPrimitive>,
}

impl GuiRenderer {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let context = Context::default();

        let mut state = State::new(event_loop);
        state.set_max_texture_side(max_texture_size);
        state.set_pixels_per_point(scale_factor);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width * 2, height * 2],
            pixels_per_point: scale_factor,
        };

        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();

        Self {
            context,
            state,
            screen_descriptor,
            renderer,
            textures,
            paint_jobs: Vec::new(),
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        _ = self.state.on_event(&self.context, event);
    }

    pub fn prepare(&mut self, gui: &mut Gui, window: &winit::window::Window, crab8: &mut Crab8) {
        let raw_input = self.state.take_egui_input(window);
        let output = self.context.run(raw_input, |context| {
            gui.render(context, crab8);
        });

        self.textures.append(output.textures_delta);
        self.state
            .handle_platform_output(window, &self.context, output.platform_output);
        self.paint_jobs = self.context.tessellate(output.shapes);
    }

    pub fn render(
        &mut self,
        encoder: &mut CommandEncoder,
        render_target: &TextureView,
        context: &PixelsContext,
    ) {
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }

        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Scope mutable borrow of self.renderer
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("gui"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut render_pass, &self.paint_jobs, &self.screen_descriptor);
        }

        let textures = std::mem::take(&mut self.textures);

        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        assert!(width > 0, "Resizing to less than 1 pixels wide");
        assert!(height > 0, "Resizing to less than 1 pixels tall");

        self.screen_descriptor.size_in_pixels = [width, height];
    }

    pub fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }
}
