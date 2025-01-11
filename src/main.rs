#![no_std]
#![no_main]

mod console;
mod cpu;
mod gui;
mod terminal;
mod drivers {
    pub mod virtio;
}

use core::panic::PanicInfo;
use drivers::virtio::GPU;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(mut gpu) = GPU.try_lock() {
        gpu.clear_screen(0xFF0000); // Red screen of death
        gpu.flush();
    }
    loop {
        unsafe {
            cpu::wfi();
        }
    }
}

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    // Initialize core systems
    unsafe {
        cpu::disable_interrupts();
    }

    // Boot sequence
    boot_sequence();

    // Main system initialization
    init_system();

    // Enable interrupts and run terminal
    unsafe {
        cpu::enable_interrupts();
    }
    terminal::run()
}

fn boot_sequence() {
    drivers::virtio::init();

    if let Some(mut gpu) = GPU.try_lock() {
        // Boot animation
        for i in 0..=100 {
            gpu.clear_screen(0x000000);
            draw_boot_screen(&mut gpu, i);
            gpu.flush();
            cpu::delay(10000);
        }
    }
}

fn init_system() {
    console::init();
    gui::init();
    cpu::init();
}

fn draw_boot_screen(gpu: &mut drivers::virtio::VirtIOGPU, progress: u32) {
    // Draw logo
    gpu.draw_rect(462, 284, 100, 100, 0x00FF00);

    // Draw progress bar background
    gpu.draw_rect(312, 484, 400, 30, 0x333333);

    // Draw progress bar
    let progress_width = (progress as u32 * 396) / 100;
    gpu.draw_rect(314, 486, progress_width, 26, 0x00FF00);
}
