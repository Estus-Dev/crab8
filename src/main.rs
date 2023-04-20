use std::process::Command;

fn main() {
    Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("bevy_chip8")
        .output()
        .expect("Expected cargo to be available/for this crate to be run inside the workspace.");
}
