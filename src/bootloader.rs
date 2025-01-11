//! Bootloader with Nyan cat animation

use crate::{console, cpu};

const BOOT_DELAY: u32 = 1_000_000;
const FRAME_DELAY: u32 = 100_000;

const NYAN_CAT: &str = r#"
 +      o     +              o    +
     +             o     +       +
 o          +
     o  +           +        +
 +        o     o       +        o
 ~-_-_-_-_-_-_-,------,      o
 _-_-_-_-_-_-_-|   /\_/\
 -_-_-_-_-_-_-~|__( ^ .^)  +     +
 _-_-_-_-_-_-_-""  ""
     N y a n N i x   v 0.1.0
 +      o         o   +       o
     +         +
 o        o         o      o     +
     o           +
 +      +     o        o      +    "#;

pub fn show_boot_sequence() {
    console::clear_screen();

    // Show initial message
    console::puts("\x1B[1;35m"); // Magenta
    console::puts("NyanNix Bootloader v0.1.0\n");
    console::puts("------------------------\n\n");
    console::puts("\x1B[0m");

    // Boot steps
    show_step("Initializing hardware", true);
    cpu::delay(BOOT_DELAY);

    show_step("Loading kernel", true);
    cpu::delay(BOOT_DELAY);

    show_step("Mounting filesystems", true);
    cpu::delay(BOOT_DELAY);

    // Nyan Cat Animation
    for _ in 0..3 {
        console::clear_screen();
        console::puts("\x1B[1;35m"); // Magenta
        console::puts(NYAN_CAT);
        console::puts("\x1B[0m");
        cpu::delay(FRAME_DELAY);
    }

    // Boot complete
    console::clear_screen();
    console::puts("\x1B[1;32m");
    console::puts("Boot complete! Starting NyanNix GUI...\n");
    console::puts("\x1B[0m");
    cpu::delay(BOOT_DELAY);
}

fn show_step(step: &str, success: bool) {
    console::puts("[ ");
    if success {
        console::puts("\x1B[1;32m OK \x1B[0m");
    } else {
        console::puts("\x1B[1;31mFAIL\x1B[0m");
    }
    console::puts(" ] ");
    console::puts(step);
    console::puts("\n");
}
