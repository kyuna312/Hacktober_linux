//! Memory management

/// Memory page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Physical memory manager
pub struct PhysicalMemory {
    start: usize,
    size: usize,
}

impl PhysicalMemory {
    pub const fn new(start: usize, size: usize) -> Self {
        Self { start, size }
    }
}

/// Initialize memory subsystem
pub fn init() {
    unsafe {
        // Clear BSS
        extern "C" {
            static mut __bss_start: u8;
            static mut __bss_end: u8;
        }

        let bss_len = {
            let start = &raw const __bss_start as *const u8 as usize;
            let end = &raw const __bss_end as *const u8 as usize;
            end - start
        };

        core::ptr::write_bytes(&raw mut __bss_start as *mut u8, 0, bss_len);
    }
}
