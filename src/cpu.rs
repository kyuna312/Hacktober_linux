//! CPU control functions

#[inline(always)]
pub unsafe fn wfi() {
    core::arch::asm!("wfi");
}

#[inline(always)]
pub unsafe fn enable_interrupts() {
    core::arch::asm!("msr daifclr, #2");
}

#[inline(always)]
pub unsafe fn disable_interrupts() {
    core::arch::asm!("msr daifset, #2");
}

pub fn init() {
    unsafe {
        disable_interrupts();
        // Basic CPU initialization here
        enable_interrupts();
    }
}

pub fn delay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop") };
    }
}
