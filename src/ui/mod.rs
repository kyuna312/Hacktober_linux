use crate::drivers::font::FONT_HEIGHT;
use crate::drivers::virtio::GPU;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Terminal {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    buffer: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
}

impl Terminal {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            buffer: Vec::new(),
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn draw(&self) {
        let mut gpu = GPU.lock();

        // Draw terminal background
        gpu.draw_rect(self.x, self.y, self.width, self.height, 0x00FFFFFF);

        // Draw terminal border
        gpu.draw_rect(self.x, self.y, self.width, 1, 0x00000000);
        gpu.draw_rect(self.x, self.y, 1, self.height, 0x00000000);
        gpu.draw_rect(self.x + self.width - 1, self.y, 1, self.height, 0x00000000);
        gpu.draw_rect(self.x, self.y + self.height - 1, self.width, 1, 0x00000000);

        // Draw terminal content
        for (i, line) in self.buffer.iter().enumerate() {
            gpu.draw_text(
                self.x + 5,
                self.y + 5 + (i as u32 * FONT_HEIGHT as u32),
                line,
                0x00000000,
            );
        }
    }

    pub fn handle_key(&mut self, key: char) {
        if key == '\n' {
            self.cursor_y += 1;
            self.cursor_x = 0;
            self.buffer.push(String::new());
        } else {
            if self.cursor_y >= self.buffer.len() {
                self.buffer.push(String::new());
            }
            let line = &mut self.buffer[self.cursor_y];
            line.push(key);
            self.cursor_x += 1;
        }
        self.draw();
    }

    pub fn handle_mouse(&mut self, _x: i32, _y: i32, _buttons: u8) {
        // Handle mouse input if needed
    }
}
