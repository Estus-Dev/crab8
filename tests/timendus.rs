//! Automatically run tests from the Timendus test suite.

use std::str::FromStr;

use crab8::{conditions::StopCondition::*, screen::Screen, Crab8};

#[test]
fn timendus_1_chip8_logo() {
    let test_rom = include_bytes!("./timendus-test-suite/bin/1-chip8-logo.ch8");
    let expected_screen = include_str!("./expected/1-chip8-logo.ch8s");
    let expected_screen = Screen::from_str(expected_screen).unwrap();
    let expected_registers = "0-F: 30 10 00 00 00 00 00 00 00 00 00 00 00 00 00 00 ".to_owned()
        + "D: 00 S: 00 CS: 00 I: 02F5 (CE 87 03 03) PC: 024E (12 4E 0F 02) - jump 0x24E";

    let mut crab8 = Crab8::new();

    crab8.load(test_rom);
    crab8.run_to_completion(&[MaxFrames(100), MaxCycles(1000)]);

    assert_eq!(expected_screen, crab8.screen);
    assert_eq!(expected_registers, crab8.dump_registers());
}

#[test]
fn timendus_2_ibm_logo() {
    let test_rom = include_bytes!("./timendus-test-suite/bin/2-ibm-logo.ch8");
    let expected_screen = include_str!("./expected/2-ibm-logo.ch8s");
    let expected_screen = Screen::from_str(expected_screen).unwrap();
    let expected_registers = "0-F: 31 08 00 00 00 00 00 00 00 00 00 00 00 00 00 00".to_owned()
        + " D: 00 S: 00 CS: 00 I: 0275 (E5 05 E2 00) PC: 0228 (12 28 FF 00) - jump 0x228";

    let mut crab8 = Crab8::new();

    crab8.load(test_rom);
    crab8.run_to_completion(&[MaxFrames(100), MaxCycles(1000)]);

    assert_eq!(expected_screen, crab8.screen);
    assert_eq!(expected_registers, crab8.dump_registers());
}

#[test]
fn timendus_3_corax_plus() {
    let test_rom = include_bytes!("./timendus-test-suite/bin/3-corax+.ch8");
    let expected_screen = include_str!("./expected/3-corax+.ch8s");
    let expected_screen = Screen::from_str(expected_screen).unwrap();
    let expected_registers = "0-F: FB 03 07 00 00 2A 05 EC 32 36 3B 10 00 00 00 00".to_owned()
        + " D: 00 S: 00 CS: 00 I: 0465 (00 A0 C0 80) PC: 045C (14 5C 01 03) - jump 0x45C";

    let mut crab8 = Crab8::new();

    crab8.load(test_rom);
    crab8.run_to_completion(&[MaxFrames(1000), MaxCycles(10000)]);

    assert_eq!(expected_screen, crab8.screen);
    assert_eq!(expected_registers, crab8.dump_registers());
}

// TODO: Pass Display Wait quirk
#[test]
fn timendus_5_quirks() {
    let test_rom = include_bytes!("./timendus-test-suite/bin/5-quirks.ch8");
    let expected_screen = include_str!("./expected/5-quirks.ch8s");
    let expected_screen = Screen::from_str(expected_screen).unwrap();
    let expected_registers = "0-F: 40 40 40 00 3C 08 00 3C 0C 00 58 64 84 38 1A 00".to_owned()
        + " D: 00 S: 00 CS: 00 I: 062D (E0 80 C0 80) PC: 05D2 (F0 0A 22 02) - v0 := key";

    let mut crab8 = Crab8::new();

    crab8.load(test_rom);

    // TODO: Set quirks automatically
    crab8.quirks.vf_reset = true;
    crab8.quirks.display_wait = true;

    // Select default CHIP-8 platform
    crab8.memory.set(0x1FF.into(), 1);

    // Run until a keypress is required because the test is meant to be rerun
    let reason = crab8.run_to_completion(&[MaxFrames(1000), MaxCycles(10000), PromptForInput]);
    assert!(matches!(reason, Some(PromptForInput)));

    assert_eq!(expected_screen, crab8.screen);
    assert_eq!(expected_registers, crab8.dump_registers());
}
