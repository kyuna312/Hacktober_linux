//! Interrupt handling

/// Initialize interrupt handling
pub fn init() {
    unsafe {
        // Disable interrupts during init
        core::arch::asm!("msr daifset, #2");

        // Set up exception vectors
        core::arch::asm!("msr vbar_el1, {}", in(reg) &VECTORS as *const _ as usize);

        // Enable interrupts
        core::arch::asm!("msr daifclr, #2");
    }
}

#[repr(align(2048))]
struct ExceptionVectors([u32; 16]);

static VECTORS: ExceptionVectors = ExceptionVectors([0xd503201f; 16]);
