use chip_8::memory::CHAR_SPRITE_WIDTH;
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

    for (column, character) in (0x0..=0xF).enumerate() {
        let address = Character::try_from(character).unwrap().address();
        let x_offset = 5 * column as u8;
        let y_offset = 6 * (x_offset / 65);
        let x_offset = x_offset % 65;

        chip8.exec(StoreAddress(address));
        chip8.exec(Store(V0, x_offset));
        chip8.exec(Store(V1, y_offset));
        chip8.exec(Draw(V0, V1, CHAR_SPRITE_WIDTH as u8));
    }

    chip8.exec(Store(V0, 29));
    chip8.exec(Store(V1, 14));
    chip8.exec(StoreAddress(CharC.address()));
    chip8.exec(Draw(V0, V1, 0x05));
    chip8.exec(StoreAddress(Char8.address()));
    chip8.exec(Draw(V0, V1, 0x05));

    chip8.input = Input::build()
        .set_pressed(Key5, true)
        .set_pressed(KeyF, true)
        .build();

    println!("{chip8}");

    Ok(())
}
