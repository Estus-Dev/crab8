use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize console_log");

    super::main();
}
