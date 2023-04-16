use chip_8::*;

fn main() {
    use Register::*;

    let mut chip8 = Chip8::default();

    chip8.registers.set(V2, 8);
    chip8.registers.set(V8, 16);
    chip8.registers.set(VF, 255);

    println!("{chip8}");
}
