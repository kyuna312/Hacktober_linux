#![no_std]
#![no_main]

mod console;
mod cpu;
mod drivers;
mod terminal;

use core::panic::PanicInfo;

const NYAN_LOGO: &str = r#"
    /\___/\
   (  o o  )
   (  =^=  )
    (---)
  /\__/\__/\    NyanNix v1.0.0
 /          \   Welcome to the cutest OS!
|            |
|   |  |  |  |
 \  |  |  | /   Press Enter to continue...
  ~~~~~~~~~~
"#;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    console::puts("\x1B[1;31mNyan panic! Something went wrong :(\x1B[0m\n");
    loop {
        unsafe {
            cpu::wfi();
        }
    }
}

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    // Initialize basic hardware
    unsafe {
        cpu::disable_interrupts();
    }
    console::init();

    // Show boot logo
    console::clear_screen();
    console::puts("\x1B[1;35m"); // Bright magenta
    console::puts(NYAN_LOGO);
    console::puts("\x1B[0m\n");

    // Wait for Enter key
    loop {
        if console::getc() == b'\r' {
            break;
        }
        unsafe {
            cpu::wfi();
        }
    }

    // Initialize display
    console::puts("\x1B[1;35mInitializing display...\x1B[0m\n");
    drivers::virtio::init();

    // Initialize terminal
    terminal::init();

    // Show welcome message
    terminal::puts("\x1B[1;35m=^.^= Welcome to NyanNix! =^.^=\x1B[0m\n\n");
    terminal::puts("Type 'help' for available commands\n");
    terminal::puts("Type 'nyan' for a surprise!\n\n");
    terminal::puts("$ ");

    // Enable interrupts and start terminal
    unsafe {
        cpu::enable_interrupts();
    }
    terminal::run()
}
