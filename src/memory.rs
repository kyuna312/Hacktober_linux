//! Memory management

use core::ptr;

/// Initialize memory subsystem
pub unsafe fn init() {
    // Clear BSS section
    extern "C" {
        static mut __bss_start: u8;
        static mut __bss_end: u8;
    }

    let bss_len = {
        let start = &raw const __bss_start as *const u8 as usize;
        let end = &raw const __bss_end as *const u8 as usize;
        end - start
    };

    ptr::write_bytes(&raw mut __bss_start as *mut u8, 0, bss_len);
}
