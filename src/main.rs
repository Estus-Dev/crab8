use chip_8::*;

fn main() {
    use Register::*;

    let mut chip8 = Chip8::default();

    chip8.set_register(V2, 8);
    chip8.set_register(V8, 16);
    chip8.set_register(VF, 255);

    println!("{chip8}");
}
