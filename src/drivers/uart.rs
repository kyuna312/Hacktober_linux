//! PL011 UART driver

use core::ptr::{read_volatile, write_volatile};
use spin::Mutex;

const UART0_BASE: usize = 0x0900_0000;
const UART0_DR: *mut u32 = UART0_BASE as *mut u32;
const UART0_FR: *mut u32 = (UART0_BASE + 0x18) as *mut u32;
const UART0_IBRD: *mut u32 = (UART0_BASE + 0x24) as *mut u32;
const UART0_FBRD: *mut u32 = (UART0_BASE + 0x28) as *mut u32;
const UART0_LCRH: *mut u32 = (UART0_BASE + 0x2C) as *mut u32;
const UART0_CR: *mut u32 = (UART0_BASE + 0x30) as *mut u32;
const UART0_IMSC: *mut u32 = (UART0_BASE + 0x38) as *mut u32;

pub struct Uart {
    initialized: bool,
}

impl Uart {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        unsafe {
            // Disable UART
            write_volatile(UART0_CR, 0);

            // Setup UART clock
            write_volatile(UART0_IBRD, 26);
            write_volatile(UART0_FBRD, 3);

            // Enable FIFO & 8-N-1
            write_volatile(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

            // Enable UART, RX, TX
            write_volatile(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));

            // Enable receive interrupt
            write_volatile(UART0_IMSC, 1 << 4);
        }

        self.initialized = true;
    }

    pub fn putc(&self, c: u8) {
        unsafe {
            while (read_volatile(UART0_FR) & (1 << 5)) != 0 {}
            write_volatile(UART0_DR, c as u32);
        }
    }

    pub fn getc(&self) -> Option<u8> {
        unsafe {
            if (read_volatile(UART0_FR) & (1 << 4)) == 0 {
                Some((read_volatile(UART0_DR) & 0xFF) as u8)
            } else {
                None
            }
        }
    }
}

pub static UART: Mutex<Uart> = Mutex::new(Uart::new());
