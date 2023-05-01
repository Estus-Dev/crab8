use winit::error::OsError;

fn main() -> Result<(), OsError> {
    crab8_frontend::build_window()
}
