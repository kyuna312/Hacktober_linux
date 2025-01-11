//! Terminal implementation

use crate::console;
use crate::drivers::virtio::GPU;
use spin::Mutex;

const TERM_WIDTH: u32 = 80;
const TERM_HEIGHT: u32 = 25;
const CHAR_WIDTH: u32 = 12;
const CHAR_HEIGHT: u32 = 16;

pub struct Terminal {
    cursor_x: u32,
    cursor_y: u32,
    fg_color: u32,
    bg_color: u32,
    buffer: [[char; 80]; 25],
    input_buffer: [char; 80],
    input_pos: usize,
}

impl Terminal {
    pub const fn new() -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            fg_color: 0xFF69B4, // Hot pink
            bg_color: 0x000000, // Black
            buffer: [[' '; 80]; 25],
            input_buffer: [' '; 80],
            input_pos: 0,
        }
    }

    pub fn init(&mut self) {
        if let Some(mut gpu) = GPU.try_lock() {
            gpu.clear_screen(self.bg_color);
            self.redraw();
            gpu.flush();
        }
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            '\r' => self.cursor_x = 0,
            '\x08' => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1
                }
            }
            _ => {
                if self.cursor_x < TERM_WIDTH {
                    self.buffer[self.cursor_y as usize][self.cursor_x as usize] = c;
                    self.draw_char(self.cursor_x, self.cursor_y);
                    self.cursor_x += 1;
                }
                if self.cursor_x >= TERM_WIDTH {
                    self.new_line();
                }
            }
        }
    }

    fn draw_char(&mut self, x: u32, y: u32) {
        if let Some(mut gpu) = GPU.try_lock() {
            let px = 20 + x * CHAR_WIDTH;
            let py = 40 + y * CHAR_HEIGHT;
            let c = self.buffer[y as usize][x as usize];

            // Clear background
            gpu.draw_rect(px, py, CHAR_WIDTH, CHAR_HEIGHT, self.bg_color);

            // Draw character if it's not a space
            if c != ' ' {
                gpu.draw_rect(
                    px + 2,
                    py + 2,
                    CHAR_WIDTH - 4,
                    CHAR_HEIGHT - 4,
                    self.fg_color,
                );
            }

            gpu.flush();
        }
    }

    fn new_line(&mut self) {
        self.cursor_x = 0;
        if self.cursor_y < TERM_HEIGHT - 1 {
            self.cursor_y += 1;
        } else {
            self.scroll_up();
        }
    }

    fn backspace(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.buffer[self.cursor_y as usize][self.cursor_x as usize] = ' ';
            self.draw_char(self.cursor_x, self.cursor_y);
        }
    }

    fn scroll_up(&mut self) {
        // Move all lines up
        for y in 1..TERM_HEIGHT as usize {
            for x in 0..TERM_WIDTH as usize {
                self.buffer[y - 1][x] = self.buffer[y][x];
            }
        }

        // Clear bottom line
        for x in 0..TERM_WIDTH as usize {
            self.buffer[TERM_HEIGHT as usize - 1][x] = ' ';
        }

        // Redraw entire screen
        self.redraw();
    }

    fn redraw(&mut self) {
        if let Some(mut gpu) = GPU.try_lock() {
            // Clear screen
            gpu.draw_rect(
                20,
                40,
                TERM_WIDTH * CHAR_WIDTH,
                TERM_HEIGHT * CHAR_HEIGHT,
                self.bg_color,
            );

            // Redraw all characters
            for y in 0..TERM_HEIGHT {
                for x in 0..TERM_WIDTH {
                    let c = self.buffer[y as usize][x as usize];
                    if c != ' ' {
                        self.draw_char(x, y);
                    }
                }
            }

            gpu.flush();
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }

    pub fn process_command(&mut self) {
        // Convert input buffer to command string
        let mut cmd = [0u8; 80];
        let mut cmd_len = 0;

        for i in 0..self.input_pos {
            if self.input_buffer[i] != ' ' || cmd_len > 0 {
                cmd[cmd_len] = self.input_buffer[i] as u8;
                cmd_len += 1;
            }
        }

        // Clear input buffer
        self.input_pos = 0;
        self.input_buffer = [' '; 80];

        // Convert command bytes to str
        if let Ok(cmd_str) = core::str::from_utf8(&cmd[..cmd_len]) {
            match cmd_str.trim() {
                "help" => {
                    self.write_string("\nAvailable commands:\n");
                    self.write_string("  help     - Show this help message\n");
                    self.write_string("  ls       - List files\n");
                    self.write_string("  clear    - Clear screen\n");
                    self.write_string("  nyan     - Show Nyan Cat\n");
                    self.write_string("  version  - Show NyanNix version\n");
                }
                "ls" => {
                    self.write_string("\nNyanNix File System:\n");
                    self.write_string("  boot/\n");
                    self.write_string("  home/\n");
                    self.write_string("  nya/\n");
                }
                "clear" => {
                    if let Some(mut gpu) = GPU.try_lock() {
                        gpu.clear_screen(self.bg_color);
                        gpu.flush();
                    }
                    self.cursor_x = 0;
                    self.cursor_y = 0;
                }
                "nyan" => {
                    self.write_string("\n");
                    self.write_string("  /\\___/\\  ~\n");
                    self.write_string(" ( ^ . ^ )~\n");
                    self.write_string("  > ' ' <\n");
                    self.write_string("   ~~~~~\n");
                }
                "version" => {
                    self.write_string("\nNyanNix v1.0.0\n");
                }
                "" => {}
                _ => {
                    self.write_string("\nCommand not found: ");
                    self.write_string(cmd_str);
                    self.write_string("\n");
                }
            }
        }
        self.write_string("$ ");
    }

    pub fn handle_input(&mut self, c: char) {
        match c {
            '\n' | '\r' => {
                self.write_char('\n');
                self.process_command();
            }
            '\x08' => {
                // Backspace
                if self.input_pos > 0 {
                    self.input_pos -= 1;
                    self.input_buffer[self.input_pos] = ' ';
                    self.backspace();
                }
            }
            _ => {
                if self.input_pos < 79 && c.is_ascii() {
                    self.input_buffer[self.input_pos] = c;
                    self.input_pos += 1;
                    self.write_char(c);
                }
            }
        }
    }
}

pub static TERM: Mutex<Terminal> = Mutex::new(Terminal::new());

pub fn init() {
    TERM.lock().init();
}

pub fn puts(s: &str) {
    TERM.lock().write_string(s);
}

pub fn run() -> ! {
    loop {
        let c = console::getc() as char;
        TERM.lock().handle_input(c);
    }
}
