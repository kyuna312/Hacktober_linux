//! Console driver for UART

const UART_BASE: usize = 0x0900_0000;
const UART_FR: usize = UART_BASE + 0x18;
const UART_DR: usize = UART_BASE;

pub fn init() {
    unsafe {
        // Initialize UART
        core::ptr::write_volatile((UART_BASE + 0x30) as *mut u32, 0x301);
    }
}

pub fn putc(c: u8) {
    unsafe {
        // Wait until UART is ready to transmit
        while (core::ptr::read_volatile(UART_FR as *const u32) & (1 << 5)) != 0 {
            core::hint::spin_loop();
        }
        core::ptr::write_volatile(UART_DR as *mut u8, c);
    }
}

pub fn getc() -> u8 {
    unsafe {
        // Wait until UART has received data
        while (core::ptr::read_volatile(UART_FR as *const u32) & (1 << 4)) != 0 {
            core::hint::spin_loop();
        }
        core::ptr::read_volatile(UART_DR as *const u8)
    }
}

pub fn puts(s: &str) {
    for c in s.bytes() {
        putc(c);
    }
}

pub fn clear_screen() {
    puts("\x1B[2J\x1B[H");
}
