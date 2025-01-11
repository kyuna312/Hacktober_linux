#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

mod drivers;
mod ui;

use drivers::{GPU, KEYBOARD, MOUSE};
use ui::Terminal;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Change parameter type from char to u32
#[no_mangle]
pub extern "C" fn keyboard_handler(keycode: u32) {
    // Convert keycode to char if needed
    if let Some(key) = char::from_u32(keycode) {
        KEYBOARD.lock().push_key(key);
    }
}

#[no_mangle]
pub extern "C" fn mouse_handler(dx: i32, dy: i32, buttons: u8) {
    MOUSE.lock().handle_interrupt(dx, dy, buttons);
}

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    // Initialize heap
    unsafe {
        let heap_start = 0x4100_0000 as *mut u8;
        let heap_size = 1024 * 1024; // 1MB
        ALLOCATOR.lock().init(heap_start, heap_size);
    }

    // Initialize hardware
    GPU.lock().init();
    KEYBOARD.lock().init();
    MOUSE.lock().init();

    // Create terminal
    let mut terminal = Terminal::new(50, 50, 700, 500);

    // Draw initial UI
    let mut gpu = GPU.lock();
    gpu.clear_screen(0x00336699);
    terminal.draw();
    drop(gpu); // Release the lock

    // Main event loop
    loop {
        // Handle keyboard input
        if let Some(key) = KEYBOARD.lock().read_key() {
            terminal.handle_key(key);
        }

        // Handle mouse input
        if let Some((x, y, buttons)) = MOUSE.lock().poll() {
            terminal.handle_mouse(x, y, buttons);

            // Draw cursor
            let mut gpu = GPU.lock();
            gpu.draw_rect(x as u32, y as u32, 5, 5, 0x00FFFFFF);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    loop {}
}
