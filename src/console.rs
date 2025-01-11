//! Console driver for UART

use core::ptr::{read_volatile, write_volatile};
use spin::Mutex;

const UART0_BASE: usize = 0x0900_0000;
const UART0_DR: *mut u32 = UART0_BASE as *mut u32;
const UART0_FR: *mut u32 = (UART0_BASE + 0x18) as *mut u32;

pub static CONSOLE: Mutex<()> = Mutex::new(());

pub fn init() {
    // Console initialization code
}

pub fn putc(c: u8) {
    unsafe {
        while (read_volatile(UART0_FR) & (1 << 5)) != 0 {}
        write_volatile(UART0_DR, c as u32);
    }
}

pub fn getc() -> Option<u8> {
    unsafe {
        if (read_volatile(UART0_FR) & (1 << 4)) == 0 {
            Some((read_volatile(UART0_DR) & 0xFF) as u8)
        } else {
            None
        }
    }
}

pub fn has_input() -> bool {
    unsafe { (read_volatile(UART0_FR) & (1 << 4)) == 0 }
}

pub fn puts(s: &str) {
    let _lock = CONSOLE.lock();
    for c in s.bytes() {
        putc(c);
    }
}

pub fn move_cursor(x: u16, y: u16) {
    let _lock = CONSOLE.lock();
    // Convert numbers to strings manually since we can't use format!
    puts("\x1B[");
    for c in y.to_string().bytes() {
        putc(c);
    }
    putc(b';');
    for c in x.to_string().bytes() {
        putc(c);
    }
    putc(b'H');
}

// Helper function to convert u16 to string
trait ToString {
    fn to_string(&self) -> &'static str;
}

impl ToString for u16 {
    fn to_string(&self) -> &'static str {
        match *self {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "11",
            12 => "12",
            13 => "13",
            14 => "14",
            15 => "15",
            16 => "16",
            17 => "17",
            18 => "18",
            19 => "19",
            20 => "20",
            // Add more if needed
            _ => "0",
        }
    }
}

pub fn clear_screen() {
    let _lock = CONSOLE.lock();
    puts("\x1B[2J");
    puts("\x1B[H");
}
