use wasm_bindgen::prelude::*;
use winit::{platform::web::WindowExtWebSys, window::Window};

#[wasm_bindgen(start)]
pub async fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize console_log");

    wasm_bindgen_futures::spawn_local(super::run());
}

pub fn insert_canvas(winit_window: &Window) {
    let canvas = winit_window.canvas();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    body.append_child(&canvas).unwrap();
}
