use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;

pub static MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub x: i32,
    pub y: i32,
    pub buttons: u8,
}

pub struct Mouse {
    state: MouseState,
}

impl Mouse {
    const fn new() -> Self {
        Self {
            state: MouseState {
                x: 0,
                y: 0,
                buttons: 0,
            },
        }
    }

    pub fn init(&mut self) {
        if INITIALIZED.load(Ordering::SeqCst) {
            return;
        }
        INITIALIZED.store(true, Ordering::SeqCst);
    }

    // This is now used internally by the interrupt handler
    #[inline]
    fn update_state(&mut self, dx: i32, dy: i32, buttons: u8) {
        self.state.x = (self.state.x + dx).clamp(0, 799);
        self.state.y = (self.state.y + dy).clamp(0, 599);
        self.state.buttons = buttons;
    }

    pub fn poll(&self) -> Option<(i32, i32, u8)> {
        Some((self.state.x, self.state.y, self.state.buttons))
    }

    // Add this method to handle interrupts
    pub fn handle_interrupt(&mut self, dx: i32, dy: i32, buttons: u8) {
        self.update_state(dx, dy, buttons);
    }
}
