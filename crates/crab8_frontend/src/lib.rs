mod screen;
mod window;

pub fn run() {
    window::build_window().expect("Failed to build window");
}
