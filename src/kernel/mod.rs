//! Kernel core functionality

pub mod device;
pub mod interrupt;
pub mod memory;
pub mod process;

/// Initialize the kernel
pub fn init() {
    // Initialize kernel subsystems
    interrupt::init();
    memory::init();
    process::init();
    device::init();
}

/// Kernel information
pub fn version() -> &'static str {
    "NyanNix v0.1.0"
}
