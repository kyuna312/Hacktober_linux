//! CPU control functions

#[inline(always)]
pub unsafe fn nop() {
    core::arch::asm!("nop");
}

#[inline(always)]
pub unsafe fn wfi() {
    core::arch::asm!("wfi");
}

#[inline(always)]
pub unsafe fn disable_interrupts() {
    core::arch::asm!("msr daifset, #2");
}

#[inline(always)]
pub unsafe fn enable_interrupts() {
    core::arch::asm!("msr daifclr, #2");
}

#[inline(always)]
pub fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[inline(always)]
pub fn wait_for_interrupt() {
    unsafe {
        wfi();
    }
}

pub fn init() {
    // CPU initialization code
}
