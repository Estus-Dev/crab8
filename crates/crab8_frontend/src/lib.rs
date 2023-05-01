use winit::{
    dpi::LogicalSize,
    error::OsError,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WINDOW_WIDTH: f64 = 1024.0;
const WINDOW_HEIGHT: f64 = 512.0;

pub fn build_window() -> Result<(), OsError> {
    let event_loop = EventLoop::new();
    let window_size = LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let _winit_window = WindowBuilder::new()
        .with_title("CRAB-8")
        .with_min_inner_size(window_size)
        .with_inner_size(window_size)
        .build(&event_loop)
        .expect("Failed to build winit window");

    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}
