use core::ptr::{read_volatile, write_volatile};
use spin::Mutex;

// VirtIO MMIO registers
const VIRTIO_MMIO_BASE: usize = 0x0A00_0000;
const VIRTIO_STATUS: usize = VIRTIO_MMIO_BASE + 0x70;
const VIRTIO_CONTROL: usize = VIRTIO_MMIO_BASE + 0x100;

// Framebuffer configuration
const FB_WIDTH: u32 = 1024;
const FB_HEIGHT: u32 = 768;
const FB_BASE: usize = 0x4000_0000;

#[repr(C)]
#[derive(Debug)]
pub struct VirtIOGPU {
    width: u32,
    height: u32,
    initialized: bool,
}

impl VirtIOGPU {
    pub const fn new() -> Self {
        Self {
            width: FB_WIDTH,
            height: FB_HEIGHT,
            initialized: false,
        }
    }

    #[allow(dead_code)]
    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        unsafe {
            // Reset device
            write_volatile((VIRTIO_STATUS) as *mut u32, 0);

            // Initialize device
            write_volatile((VIRTIO_STATUS) as *mut u32, 1);
            write_volatile((VIRTIO_STATUS) as *mut u32, 2);
            write_volatile((VIRTIO_STATUS) as *mut u32, 8);
            write_volatile((VIRTIO_STATUS) as *mut u32, 4);

            // Initialize display
            self.init_display();

            self.initialized = true;

            // Initial screen setup
            self.clear_screen(0x0000FF);
            self.flush();
        }
    }

    unsafe fn init_display(&self) {
        // Set display mode
        write_volatile((VIRTIO_CONTROL) as *mut u32, 0x100);
        write_volatile((VIRTIO_CONTROL + 4) as *mut u32, self.width);
        write_volatile((VIRTIO_CONTROL + 8) as *mut u32, self.height);

        // Wait for completion
        while read_volatile((VIRTIO_CONTROL + 12) as *const u32) & 1 != 0 {
            core::hint::spin_loop();
        }
    }

    #[allow(dead_code)]
    pub fn clear_screen(&mut self, color: u32) {
        if !self.initialized {
            return;
        }

        unsafe {
            let fb = FB_BASE as *mut u32;
            for i in 0..(self.width * self.height) as isize {
                write_volatile(fb.offset(i), color);
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        if !self.initialized {
            return;
        }

        let x_end = (x + width).min(self.width);
        let y_end = (y + height).min(self.height);

        unsafe {
            let fb = FB_BASE as *mut u32;
            for cy in y..y_end {
                for cx in x..x_end {
                    let offset = (cy * self.width + cx) as isize;
                    write_volatile(fb.offset(offset), color);
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn flush(&self) {
        if !self.initialized {
            return;
        }

        unsafe {
            write_volatile((VIRTIO_CONTROL) as *mut u32, 0x103);
            while read_volatile((VIRTIO_CONTROL + 12) as *const u32) & 1 != 0 {
                core::hint::spin_loop();
            }
        }
    }
}

pub static GPU: Mutex<VirtIOGPU> = Mutex::new(VirtIOGPU::new());

#[allow(dead_code)]
pub fn init() {
    GPU.lock().init();
}
