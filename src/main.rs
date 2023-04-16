use chip_8::prelude::*;

fn main() -> Result<(), ()> {
    use Instruction::*;
    use Register::*;

    let mut chip8 = Chip8::default();

    chip8.exec(Store(V2, 8));
    chip8.exec(Store(V8, 16));
    chip8.exec(0x6FFF);
    chip8.exec(Add(V1, 0x12));
    chip8.exec(0x7168);
    chip8.exec(0x8710);

    println!("{chip8}");

    Ok(())
}
