use super::{Menu, Window};
use crate::drivers::virtio::GPU;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

pub struct Terminal {
    pub window: Window,
    buffer: Vec<ColoredString>,
    scrollbar: Scrollbar,
    cursor_x: usize,
    cursor_y: usize,
    scroll_offset: usize,
    command_history: Vec<String>,
    history_index: isize,
    current_command: String,
    context_menu: Menu,
    selection_start: Option<(usize, usize)>,
    selection_end: Option<(usize, usize)>,
    current_dir: String,
    file_system: FileSystem,
}

struct ColoredString {
    text: String,
    color: u32,
}

pub struct FileSystem {
    root: Directory,
    current_path: String,
}

struct Directory {
    name: String,
    files: BTreeMap<String, File>,
    directories: BTreeMap<String, Directory>,
}

struct File {
    name: String,
    content: String,
}

impl Terminal {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        let mut fs = FileSystem {
            root: Directory {
                name: String::from("/"),
                files: BTreeMap::new(),
                directories: BTreeMap::new(),
            },
            current_path: String::from("/"),
        };

        // Create some initial directories and files
        fs.root.directories.insert(
            String::from("home"),
            Directory {
                name: String::from("home"),
                files: BTreeMap::new(),
                directories: BTreeMap::new(),
            },
        );

        let mut context_menu = Menu::new(0, 0);
        context_menu.add_item("Copy");
        context_menu.add_item("Paste");
        context_menu.add_item("Clear");

        Self {
            window: Window::new(x, y, width, height, "Terminal"),
            buffer: Vec::new(),
            scrollbar: Scrollbar::new(x + width - 15, y + 25, 10, height - 30),
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
            command_history: Vec::new(),
            history_index: -1,
            current_command: String::new(),
            context_menu,
            selection_start: None,
            selection_end: None,
            current_dir: String::from("/"),
            file_system: fs,
        }
    }

    pub fn print(&mut self, text: &str, color: u32) {
        let lines: Vec<&str> = text.split('\n').collect();
        for line in lines {
            self.buffer.push(ColoredString {
                text: String::from(line),
                color,
            });
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
        self.scrollbar.max_value = self.buffer.len() as u32;
        self.draw();
    }

    pub fn handle_mouse(&mut self, mouse_x: i32, mouse_y: i32, mouse_buttons: u8) {
        self.window.handle_mouse(mouse_x, mouse_y, mouse_buttons);
        self.scrollbar.handle_mouse(mouse_x, mouse_y, mouse_buttons);

        // Right-click handling
        if mouse_buttons & 2 != 0 {
            // Right button pressed
            self.context_menu.x = mouse_x as u32;
            self.context_menu.y = mouse_y as u32;
            self.context_menu.visible = true;
        }

        if let Some(item_index) = self
            .context_menu
            .handle_mouse(mouse_x, mouse_y, mouse_buttons)
        {
            match item_index {
                0 => self.copy_selection(),
                1 => self.paste_clipboard(),
                2 => self.clear_buffer(),
                _ => {}
            }
        }
    }

    pub fn execute_command(&mut self) {
        let cmd = self.current_command.trim();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.is_empty() {
            self.print("\n", 0x00000000);
            return;
        }

        match parts[0] {
            "cd" => {
                if parts.len() < 2 {
                    self.print("Usage: cd <directory>\n", 0x00FF0000);
                    return;
                }
                let new_dir = parts[1];
                if new_dir == ".." {
                    if let Some(last_slash) = self.current_dir.rfind('/') {
                        self.current_dir.truncate(last_slash + 1);
                    }
                } else {
                    self.current_dir.push_str(new_dir);
                    self.current_dir.push('/');
                }
            }
            "ls" => {
                let current = self.get_current_directory();
                for dir in &current.directories {
                    self.print(&format!("\x1b[34m{}/\x1b[0m\n", dir.name), 0x00000000);
                }
                for file in &current.files {
                    self.print(&format!("{}\n", file.name), 0x00000000);
                }
            }
            "mkdir" => {
                if parts.len() < 2 {
                    self.print("Usage: mkdir <directory>\n", 0x00FF0000);
                    return;
                }
                let current = self.get_current_directory_mut();
                current.directories.insert(
                    String::from(parts[1]),
                    Directory {
                        name: String::from(parts[1]),
                        files: BTreeMap::new(),
                        directories: BTreeMap::new(),
                    },
                );
            }
            "touch" => {
                if parts.len() < 2 {
                    self.print("Usage: touch <filename>\n", 0x00FF0000);
                    return;
                }
                let current = self.get_current_directory_mut();
                current.files.insert(
                    String::from(parts[1]),
                    File {
                        name: String::from(parts[1]),
                        content: String::new(),
                    },
                );
            }
            "clear" => self.clear_buffer(),
            "help" => self.print(
                "Available commands:\n\
                clear - Clear terminal\n\
                help - Show this help\n\
                echo [text] - Print text\n\
                color [hex] - Change text color\n\
                ls - List files\n\
                cat [file] - Show file contents\n\
                version - Show version\n",
                0x00000000,
            ),
            "echo" => {
                let text = parts[1..].join(" ");
                self.print(&format!("{}\n", text), self.current_color);
            }
            "color" => {
                if parts.len() > 1 {
                    if let Ok(color) = u32::from_str_radix(parts[1].trim_start_matches("0x"), 16) {
                        self.current_color = color;
                        self.print("Color changed\n", color);
                    } else {
                        self.print(
                            "Invalid color format. Use hex (e.g., 0xFF0000)\n",
                            0x00FF0000,
                        );
                    }
                }
            }
            "cat" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "README.md" => self.print(
                            "NyanNix Operating System\n\
                            A cute and functional OS\n",
                            0x00000000,
                        ),
                        _ => self.print("File not found\n", 0x00FF0000),
                    }
                } else {
                    self.print("Usage: cat [file]\n", 0x00FF0000);
                }
            }
            "version" => self.print("NyanNix Terminal v0.1.0\n", 0x00000000),
            _ => self.print(&format!("Unknown command: {}\n", cmd), 0x00FF0000),
        }

        self.current_command.clear();
        self.cursor_x = 0;
    }

    pub fn start_selection(&mut self, mouse_x: i32, mouse_y: i32) {
        let x = ((mouse_x - self.window.x as i32 - 5) / FONT_WIDTH as i32) as usize;
        let y = ((mouse_y - self.window.y as i32 - 25) / (FONT_HEIGHT as i32 * 3 / 2)) as usize
            + self.scroll_offset;

        if y < self.buffer.len() {
            self.selection_start = Some((x, y));
            self.selection_end = Some((x, y));
        }
    }

    pub fn update_selection(&mut self, mouse_x: i32, mouse_y: i32) {
        if self.selection_start.is_some() {
            let x = ((mouse_x - self.window.x as i32 - 5) / FONT_WIDTH as i32) as usize;
            let y = ((mouse_y - self.window.y as i32 - 25) / (FONT_HEIGHT as i32 * 3 / 2)) as usize
                + self.scroll_offset;

            if y < self.buffer.len() {
                self.selection_end = Some((x, y));
            }
        }
    }

    pub fn copy_selection(&mut self) -> String {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let mut selected_text = String::new();
            let (start_x, start_y, end_x, end_y) =
                if start.1 < end.1 || (start.1 == end.1 && start.0 <= end.0) {
                    (start.0, start.1, end.0, end.1)
                } else {
                    (end.0, end.1, start.0, start.1)
                };

            for y in start_y..=end_y {
                if y >= self.buffer.len() {
                    break;
                }
                let line = &self.buffer[y].text;
                let line_start = if y == start_y { start_x } else { 0 };
                let line_end = if y == end_y { end_x } else { line.len() };

                if line_start < line.len() {
                    selected_text.push_str(&line[line_start..line_end.min(line.len())]);
                }
                if y != end_y {
                    selected_text.push('\n');
                }
            }
            selected_text
        } else {
            String::new()
        }
    }

    pub fn paste_clipboard(&mut self, text: &str) {
        for c in text.chars() {
            if c == '\n' {
                self.execute_command();
            } else {
                self.current_command.push(c);
                self.cursor_x += 1;
            }
        }
        self.draw();
    }

    pub fn draw(&self) {
        // ... previous draw code ...

        // Draw selection
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let mut gpu = GPU.lock();
            let (start_x, start_y, end_x, end_y) =
                if start.1 < end.1 || (start.1 == end.1 && start.0 <= end.0) {
                    (start.0, start.1, end.0, end.1)
                } else {
                    (end.0, end.1, start.0, start.1)
                };

            for y in start_y..=end_y {
                if y >= self.buffer.len() || y < self.scroll_offset {
                    continue;
                }
                let screen_y = self.window.y
                    + 25
                    + ((y - self.scroll_offset) as u32 * (FONT_HEIGHT as u32 * 3 / 2));
                let line_start = if y == start_y { start_x } else { 0 };
                let line_end = if y == end_y {
                    end_x
                } else {
                    self.buffer[y].text.len()
                };

                gpu.draw_rect(
                    self.window.x + 5 + (line_start as u32 * FONT_WIDTH as u32),
                    screen_y,
                    ((line_end - line_start) as u32 * FONT_WIDTH as u32),
                    FONT_HEIGHT as u32,
                    0x400000FF,
                );
            }
        }
    }

    fn get_current_directory(&self) -> &Directory {
        let mut current = &self.file_system.root;
        for part in self.current_dir.split('/') {
            if part.is_empty() {
                continue;
            }
            if let Some(dir) = current.directories.iter().find(|d| d.name == part) {
                current = dir;
            }
        }
        current
    }

    fn get_current_directory_mut(&mut self) -> &mut Directory {
        let mut current = &mut self.file_system.root;
        for part in self.current_dir.split('/') {
            if part.is_empty() {
                continue;
            }
            if let Some(dir) = current.directories.iter_mut().find(|d| d.name == part) {
                current = dir;
            }
        }
        current
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: Directory::new("root"),
            current_path: String::from("/"),
        }
    }

    pub fn create_file(&mut self, name: &str, content: &str) -> Result<(), &'static str> {
        let current_dir = self.get_current_directory_mut()?;
        current_dir.files.insert(
            String::from(name),
            File {
                name: String::from(name),
                content: String::from(content),
            },
        );
        Ok(())
    }

    pub fn create_directory(&mut self, name: &str) -> Result<(), &'static str> {
        let current_dir = self.get_current_directory_mut()?;
        current_dir
            .directories
            .insert(String::from(name), Directory::new(name));
        Ok(())
    }

    pub fn delete(&mut self, name: &str) -> Result<(), &'static str> {
        let current_dir = self.get_current_directory_mut()?;
        if current_dir.files.remove(name).is_some()
            || current_dir.directories.remove(name).is_some()
        {
            Ok(())
        } else {
            Err("File or directory not found")
        }
    }

    pub fn list_contents(&self) -> Vec<String> {
        let mut contents = Vec::new();
        let current_dir = self.get_current_directory().unwrap();

        for dir in current_dir.directories.keys() {
            contents.push(format!("\x1b[34m{}/\x1b[0m", dir));
        }
        for file in current_dir.files.keys() {
            contents.push(file.clone());
        }
        contents
    }
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            files: BTreeMap::new(),
            directories: BTreeMap::new(),
        }
    }
}
