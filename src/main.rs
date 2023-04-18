use chip_8::prelude::*;

fn main() -> Result<(), ()> {
    use Instruction::*;

    let mut chip8 = Chip8::default();

    chip8.exec(Store(V2, 8));
    chip8.exec(Store(V8, 16));
    chip8.exec(0x6FFF);
    chip8.exec(Add(V1, 0x12));
    chip8.exec(0x7168);
    chip8.exec(0x8710);
    chip8.stack.push(0x123.try_into()?)?;
    chip8.stack.push(0x234.try_into()?)?;
    chip8.stack.push(0x345.try_into()?)?;

    chip8.input = Input::build()
        .set_pressed(Key5, true)
        .set_pressed(KeyF, true)
        .build();

    println!("{chip8}");

    Ok(())
}
