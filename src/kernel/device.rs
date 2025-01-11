//! Device management

use crate::console;

/// Initialize device subsystems
pub fn init() {
    // Initialize basic devices
    console::init();
}
