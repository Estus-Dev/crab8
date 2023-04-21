use std::process::{Command, Stdio};

fn main() {
    Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("bevy_chip8")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Expected cargo to be available/for this crate to be run inside the workspace.");
}
