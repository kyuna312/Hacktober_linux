//! Window management system

use crate::console;

pub struct Window {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub title: &'static str,
}

impl Window {
    pub const fn new(x: u16, y: u16, width: u16, height: u16, title: &'static str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            title,
        }
    }

    pub fn draw(&self) {
        // Draw top border with title
        self.move_cursor(0, 0);
        console::puts("╔");
        for _ in 0..self.width - 2 {
            console::puts("═");
        }
        console::puts("╗\n");

        // Draw title
        self.move_cursor(2, 0);
        console::puts(" ");
        console::puts(self.title);
        console::puts(" ");

        // Draw sides and content
        for i in 1..self.height - 1 {
            self.move_cursor(0, i);
            console::puts("║");
            for _ in 0..self.width - 2 {
                console::puts(" ");
            }
            console::puts("║\n");
        }

        // Draw bottom border
        self.move_cursor(0, self.height - 1);
        console::puts("╚");
        for _ in 0..self.width - 2 {
            console::puts("═");
        }
        console::puts("╝\n");
    }

    pub fn write_at(&self, x: u16, y: u16, text: &str) {
        self.move_cursor(x, y);
        console::puts(text);
    }

    fn move_cursor(&self, rel_x: u16, rel_y: u16) {
        let x = self.x + rel_x;
        let y = self.y + rel_y;
        // Using individual puts calls instead of format!
        console::puts("\x1B[");
        console::put_num(y + 1);
        console::puts(";");
        console::put_num(x + 1);
        console::puts("H");
    }
}

pub static MAIN_WINDOW: Window = Window::new(0, 0, 80, 24, "NyanNix Terminal");
pub static STATUS_WINDOW: Window = Window::new(60, 1, 19, 10, "System Status");
