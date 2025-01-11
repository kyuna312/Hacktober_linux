use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicPtr, Ordering};
use spin::Mutex;

// VirtIO GPU Memory-Mapped I/O addresses
const VIRTIO_GPU_BASE: usize = 0x0A000000;
const VIRTIO_GPU_CONTROL: usize = VIRTIO_GPU_BASE + 0x100;
const VIRTIO_GPU_DISPLAY: usize = VIRTIO_GPU_BASE + 0x200;

pub struct Display {
    width: usize,
    height: usize,
    pitch: usize,
    base: AtomicPtr<u32>,
    control: AtomicPtr<u32>,
}

unsafe impl Send for Display {}
unsafe impl Sync for Display {}

impl Display {
    pub const fn new() -> Self {
        Self {
            width: 1024,
            height: 768,
            pitch: 1024 * 4,
            base: AtomicPtr::new(VIRTIO_GPU_DISPLAY as *mut u32),
            control: AtomicPtr::new(VIRTIO_GPU_CONTROL as *mut u32),
        }
    }

    pub fn init(&self) {
        unsafe {
            // Initialize VirtIO GPU
            self.setup_display();
            self.clear_screen(0x000000); // Black background
        }
    }

    unsafe fn setup_display(&self) {
        let ctrl = self.control.load(Ordering::SeqCst);

        // Set display mode
        write_volatile(ctrl, 0x1); // VIRTIO_GPU_CMD_SET_MODE
        write_volatile(ctrl.add(1), self.width as u32);
        write_volatile(ctrl.add(2), self.height as u32);

        // Wait for command completion
        while read_volatile(ctrl.add(3)) & 1 != 0 {
            core::hint::spin_loop();
        }
    }

    pub fn clear_screen(&self, color: u32) {
        let base = self.base.load(Ordering::SeqCst);
        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let offset = y * (self.pitch / 4) + x;
                    write_volatile(base.add(offset), color);
                }
            }
        }
    }

    pub fn draw_rect(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let base = self.base.load(Ordering::SeqCst);
        let x_end = (x + width).min(self.width);
        let y_end = (y + height).min(self.height);

        unsafe {
            for cy in y..y_end {
                for cx in x..x_end {
                    let offset = cy * (self.pitch / 4) + cx;
                    write_volatile(base.add(offset), color);
                }
            }
        }
    }
}

pub static DISPLAY: Mutex<Display> = Mutex::new(Display::new());

pub fn init() {
    DISPLAY.lock().init();
}
