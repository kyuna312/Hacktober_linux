use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub static GPU: Mutex<VirtIOGPU> = Mutex::new(VirtIOGPU::new());
static INITIALIZED: AtomicBool = AtomicBool::new(false);

const FRAMEBUFFER_BASE: usize = 0x4000_0000;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

pub struct VirtIOGPU {
    framebuffer: &'static mut [u32],
    width: u32,
    height: u32,
}

impl VirtIOGPU {
    const fn new() -> Self {
        Self {
            framebuffer: &mut [],
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }

    pub fn init(&mut self) {
        if INITIALIZED.load(Ordering::SeqCst) {
            return;
        }

        self.framebuffer = unsafe {
            core::slice::from_raw_parts_mut(
                FRAMEBUFFER_BASE as *mut u32,
                (SCREEN_WIDTH * SCREEN_HEIGHT) as usize,
            )
        };

        self.clear_screen(0x00336699);
        INITIALIZED.store(true, Ordering::SeqCst);
    }

    pub fn clear_screen(&mut self, color: u32) {
        for pixel in self.framebuffer.iter_mut() {
            *pixel = color;
        }
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;

                if px >= self.width || py >= self.height {
                    continue;
                }

                let pos = (py * self.width + px) as usize;
                if pos < self.framebuffer.len() {
                    self.framebuffer[pos] = color;
                }
            }
        }
    }

    pub fn draw_text(&mut self, x: u32, y: u32, text: &str, color: u32) {
        use crate::drivers::font::FONT_8X8;

        let mut cursor_x = x;
        let mut cursor_y = y;

        for c in text.chars() {
            if c == '\n' {
                cursor_y += 8;
                cursor_x = x;
                continue;
            }

            let char_index = c as usize;
            if char_index < FONT_8X8.len() {
                let char_bitmap = FONT_8X8[char_index];
                for (row, bitmap) in char_bitmap.iter().enumerate() {
                    for col in 0..8 {
                        if (bitmap >> (7 - col)) & 1 != 0 {
                            self.draw_rect(
                                cursor_x + col as u32,
                                cursor_y + row as u32,
                                1,
                                1,
                                color,
                            );
                        }
                    }
                }
            }
            cursor_x += 8;
        }
    }
}
