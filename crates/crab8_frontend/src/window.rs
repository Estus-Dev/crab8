use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

const WIDTH: f64 = 1024.0;
const HEIGHT: f64 = 512.0;

pub struct Crab8Window {
    pub event_loop: EventLoop<()>,
    pub pixels: Pixels,
    pub winit: Window,
}

impl Crab8Window {
    pub fn new(crab8_width: u32, crab8_height: u32) -> Result<Crab8Window, pixels::Error> {
        let event_loop = EventLoop::new();
        let window_size = LogicalSize::new(WIDTH, HEIGHT);

        let winit = WindowBuilder::new()
            .with_title("CRAB-8")
            .with_min_inner_size(window_size)
            .with_inner_size(window_size)
            .build(&event_loop)
            .expect("Failed to build winit window");

        let surface = SurfaceTexture::new(crab8_width, crab8_height, &winit);

        let mut egui_state = egui_winit::State::new(&event_loop);
        egui_state.set_pixels_per_point(winit.scale_factor() as f32);

        let mut pixels = Pixels::new(crab8_width, crab8_height, surface)?;
        let window_inner_size = winit.inner_size();
        pixels.resize_surface(window_inner_size.width, window_inner_size.height)?;

        Ok(Crab8Window {
            event_loop,
            pixels,
            winit,
        })
    }
}
