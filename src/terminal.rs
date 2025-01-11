//! Terminal implementation

use crate::{console, cpu, gui};

const MAX_CMD_LEN: usize = 64;
static mut CMD_BUFFER: [u8; MAX_CMD_LEN] = [0; MAX_CMD_LEN];
static mut CMD_POS: usize = 0;

pub fn run() -> ! {
    loop {
        unsafe {
            crate::cpu::wfi();
        }
    }
}

fn show_prompt(line: u16) {
    gui::TERMINAL_WINDOW.write_at(2, line, "\x1B[1;32mNyanNix\x1B[0m:\x1B[1;34m$\x1B[0m ");
}

fn read_command(current_line: u16) -> u16 {
    unsafe {
        CMD_POS = 0;
        let mut cursor_pos: u16 = 0;

        loop {
            // Wait for keyboard input
            while !console::has_input() {
                cpu::wait_for_interrupt();
            }

            if let Some(c) = console::getc() {
                match c {
                    b'\r' | b'\n' => {
                        console::puts("\n");
                        let next_line = execute_command(current_line + 1);
                        return next_line;
                    }
                    // Handle backspace and delete
                    8 | 127 if CMD_POS > 0 => {
                        CMD_POS -= 1;
                        cursor_pos -= 1;
                        // Move cursor back and clear character
                        gui::TERMINAL_WINDOW.write_at(4 + cursor_pos, current_line, " ");
                        gui::TERMINAL_WINDOW.write_at(4 + cursor_pos, current_line, "");
                    }
                    // Handle printable characters
                    32..=126 if CMD_POS < MAX_CMD_LEN - 1 => {
                        CMD_BUFFER[CMD_POS] = c;
                        CMD_POS += 1;
                        cursor_pos += 1;
                        // Print character
                        let mut buf = [0; 1];
                        buf[0] = c;
                        gui::TERMINAL_WINDOW.write_at(
                            3 + cursor_pos,
                            current_line,
                            core::str::from_utf8(&buf).unwrap_or(""),
                        );
                    }
                    // Handle Ctrl+C
                    3 => {
                        console::puts("^C\n");
                        return current_line + 1;
                    }
                    // Handle Ctrl+D
                    4 => {
                        console::puts("exit\n");
                        return current_line + 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn execute_command(mut current_line: u16) -> u16 {
    unsafe {
        let cmd = core::str::from_utf8(&CMD_BUFFER[..CMD_POS])
            .unwrap_or("")
            .trim();
        match cmd {
            "help" => {
                show_help(current_line);
                current_line += 5;
            }
            "clear" => {
                gui::TERMINAL_WINDOW.clear_content();
                current_line = 5;
            }
            "ls" => {
                list_files(current_line);
                current_line += 4;
            }
            "version" => {
                show_version(current_line);
                current_line += 2;
            }
            "" => {
                current_line += 1;
            }
            _ => {
                gui::TERMINAL_WINDOW.write_at(
                    2,
                    current_line,
                    "\x1B[1;31mUnknown command: \x1B[0m",
                );
                gui::TERMINAL_WINDOW.write_at(19, current_line, cmd);
                current_line += 1;
            }
        }
    }
    current_line
}

fn show_help(line: u16) {
    gui::TERMINAL_WINDOW.write_at(2, line, "\x1B[1;36mAvailable commands:\x1B[0m");
    gui::TERMINAL_WINDOW.write_at(2, line + 1, "  help    - Show this help message");
    gui::TERMINAL_WINDOW.write_at(2, line + 2, "  clear   - Clear the screen");
    gui::TERMINAL_WINDOW.write_at(2, line + 3, "  ls      - List files");
    gui::TERMINAL_WINDOW.write_at(2, line + 4, "  version - Show version");
}

fn list_files(line: u16) {
    gui::TERMINAL_WINDOW.write_at(2, line, "\x1B[1;33mboot/\x1B[0m");
    gui::TERMINAL_WINDOW.write_at(2, line + 1, "\x1B[1;33mkernel/\x1B[0m");
    gui::TERMINAL_WINDOW.write_at(2, line + 2, "\x1B[1;33msystem/\x1B[0m");
}

fn show_version(line: u16) {
    gui::TERMINAL_WINDOW.write_at(2, line, "\x1B[1;35mNyanNix\x1B[0m version 0.1.0");
}
