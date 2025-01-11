//! Simple GUI implementation

use crate::console;

pub struct Window {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    title: &'static str,
    border_color: &'static str,
    title_color: &'static str,
}

impl Window {
    pub const fn new(
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        title: &'static str,
        border_color: &'static str,
        title_color: &'static str,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            title,
            border_color,
            title_color,
        }
    }

    pub fn draw(&self) {
        // Draw retro-style border with double lines
        console::puts(self.border_color);
        self.move_cursor(0, 0);
        console::puts("╔═");
        for _ in 2..self.width - 4 {
            console::puts("═");
        }
        console::puts("═╗");

        // Draw title in retro style
        self.move_cursor(2, 0);
        console::puts(self.title_color);
        console::puts("[ ");
        console::puts(self.title);
        console::puts(" ]");
        console::puts(self.border_color);

        // Draw sides with retro pattern
        for y in 1..self.height - 1 {
            self.move_cursor(0, y);
            console::puts("║");
            if y % 2 == 0 {
                console::puts("·");
            } else {
                console::puts(" ");
            }
            for _ in 2..self.width - 2 {
                console::puts(" ");
            }
            if y % 2 == 0 {
                console::puts("·");
            } else {
                console::puts(" ");
            }
            console::puts("║");
        }

        // Draw bottom border
        self.move_cursor(0, self.height - 1);
        console::puts("╚═");
        for _ in 2..self.width - 4 {
            console::puts("═");
        }
        console::puts("═╝");
        console::puts("\x1B[0m");
    }

    pub fn write_at(&self, x: u16, y: u16, text: &str) {
        if y >= self.height - 1 {
            return;
        }
        self.move_cursor(x, y);
        console::puts(text);
    }

    fn move_cursor(&self, rel_x: u16, rel_y: u16) {
        let x = self.x + rel_x;
        let y = self.y + rel_y;
        console::move_cursor(x + 1, y + 1);
    }

    pub fn clear_content(&self) {
        for y in 1..self.height - 1 {
            self.move_cursor(1, y);
            for _ in 1..self.width - 1 {
                console::puts(" ");
            }
        }
    }
}

// Retro color scheme
const RETRO_CYAN: &str = "\x1B[1;36m";
const RETRO_GREEN: &str = "\x1B[1;32m";
const RETRO_YELLOW: &str = "\x1B[1;33m";
const RETRO_MAGENTA: &str = "\x1B[1;35m";

pub static TERMINAL_WINDOW: Window =
    Window::new(0, 0, 60, 20, "NYANNIX TERMINAL", RETRO_CYAN, RETRO_GREEN);
pub static STATUS_WINDOW: Window =
    Window::new(60, 0, 20, 20, "SYSTEM", RETRO_YELLOW, RETRO_MAGENTA);

pub fn init() {
    // GUI initialization code
}

// pub fn refresh() {
//     TERMINAL_WINDOW.draw();
//     STATUS_WINDOW.draw();
// }
