use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub static KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Keyboard {
    buffer: [char; 16],
    read_pos: usize,
    write_pos: usize,
}

impl Keyboard {
    const fn new() -> Self {
        Self {
            buffer: ['\0'; 16],
            read_pos: 0,
            write_pos: 0,
        }
    }

    pub fn init(&mut self) {
        if INITIALIZED.load(Ordering::SeqCst) {
            return;
        }
        INITIALIZED.store(true, Ordering::SeqCst);
    }

    pub fn read_key(&mut self) -> Option<char> {
        if self.read_pos != self.write_pos {
            let key = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.buffer.len();
            Some(key)
        } else {
            None
        }
    }

    pub(crate) fn push_key(&mut self, key: char) {
        let next_write = (self.write_pos + 1) % self.buffer.len();
        if next_write != self.read_pos {
            self.buffer[self.write_pos] = key;
            self.write_pos = next_write;
        }
    }
}
