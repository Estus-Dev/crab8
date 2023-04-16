use chip_8::prelude::*;

fn main() -> Result<(), ()> {
    use Instruction::*;
    use Register::*;

    let mut chip8 = Chip8::default();

    chip8.exec(Store(V2, 8));
    chip8.exec(Store(V8, 16));
    chip8.exec(Instruction::try_from(0x6FFF)?);

    println!("{chip8}");

    Ok(())
}
