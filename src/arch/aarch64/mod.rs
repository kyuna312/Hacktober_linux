//! AArch64 architecture specific code

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
    core::arch::asm!("msr daifset, #15");
}

#[inline(always)]
pub unsafe fn enable_interrupts() {
    core::arch::asm!("msr daifclr, #15");
}

#[inline(always)]
pub fn current_el() -> u64 {
    let mut el: u64;
    unsafe {
        core::arch::asm!(
            "mrs {}, CurrentEL",
            out(reg) el
        );
    }
    (el >> 2) & 0x3
}
