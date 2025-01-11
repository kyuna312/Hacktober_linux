use core::ptr::{read_volatile, write_volatile};
use spin::Mutex;

const VIRTIO_MMIO_BASE: usize = 0x0A00_0000;
const VIRTIO_STATUS: usize = VIRTIO_MMIO_BASE + 0x70;
const VIRTIO_CONTROL: usize = VIRTIO_MMIO_BASE + 0x100;
const VIRTIO_NOTIFY: usize = VIRTIO_MMIO_BASE + 0x50;
const VIRTIO_CONFIG: usize = VIRTIO_MMIO_BASE + 0x200;

// Display configuration
const FB_WIDTH: u32 = 800;
const FB_HEIGHT: u32 = 600;
const FB_STRIDE: u32 = FB_WIDTH * 4;
const FB_BASE: usize = 0x4000_0000;

// VirtIO GPU commands
const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: u32 = 0x100;
const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: u32 = 0x101;
const VIRTIO_GPU_CMD_SET_SCANOUT: u32 = 0x103;
const VIRTIO_GPU_CMD_RESOURCE_FLUSH: u32 = 0x104;
const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: u32 = 0x105;

#[repr(C)]
#[derive(Debug)]
pub struct VirtIOGPU {
    width: u32,
    height: u32,
    pub initialized: bool,
    framebuffer: usize,
    resource_id: u32,
}

impl VirtIOGPU {
    pub const fn new() -> Self {
        Self {
            width: FB_WIDTH,
            height: FB_HEIGHT,
            initialized: false,
            framebuffer: FB_BASE,
            resource_id: 1,
        }
    }

    pub fn init(&mut self) {
        if self.initialized {
            return;
        }

        unsafe {
            // Reset device
            write_volatile((VIRTIO_STATUS) as *mut u32, 0);
            core::hint::spin_loop();

            // Initialize device
            write_volatile((VIRTIO_STATUS) as *mut u32, 1); // ACKNOWLEDGE
            write_volatile((VIRTIO_STATUS) as *mut u32, 2); // DRIVER
            write_volatile((VIRTIO_STATUS) as *mut u32, 8); // FEATURES_OK
            write_volatile((VIRTIO_STATUS) as *mut u32, 4); // DRIVER_OK

            // Create 2D resource
            self.send_command(
                VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
                &[
                    self.resource_id,
                    FB_WIDTH,
                    FB_HEIGHT,
                    0, // format: B8G8R8A8_UNORM
                ],
            );

            // Set scanout
            self.send_command(
                VIRTIO_GPU_CMD_SET_SCANOUT,
                &[
                    0, // scanout_id
                    self.resource_id,
                    0,
                    0, // x, y
                    FB_WIDTH,
                    FB_HEIGHT,
                ],
            );

            self.initialized = true;
            self.clear_screen(0x000000); // Black
            self.flush();
        }
    }

    fn send_command(&self, cmd: u32, data: &[u32]) {
        unsafe {
            // Write command
            write_volatile((VIRTIO_CONTROL) as *mut u32, cmd);

            // Write data
            for (i, &value) in data.iter().enumerate() {
                write_volatile((VIRTIO_CONTROL + 4 + i * 4) as *mut u32, value);
            }

            // Notify device
            write_volatile((VIRTIO_NOTIFY) as *mut u32, 0);

            // Wait for completion
            while read_volatile((VIRTIO_CONTROL + 0xFC) as *const u32) & 1 != 0 {
                core::hint::spin_loop();
            }
        }
    }

    pub fn clear_screen(&mut self, color: u32) {
        if !self.initialized {
            return;
        }

        unsafe {
            let fb = self.framebuffer as *mut u32;
            for i in 0..(self.width * self.height) as isize {
                write_volatile(fb.offset(i), color);
            }
        }
        self.update_display();
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        if !self.initialized {
            return;
        }

        let x_end = (x + width).min(self.width);
        let y_end = (y + height).min(self.height);

        unsafe {
            let fb = self.framebuffer as *mut u32;
            for cy in y..y_end {
                for cx in x..x_end {
                    let offset = (cy * self.width + cx) as isize;
                    write_volatile(fb.offset(offset), color);
                }
            }
        }
    }

    fn update_display(&mut self) {
        if !self.initialized {
            return;
        }

        // Transfer framebuffer to host
        self.send_command(
            VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D,
            &[
                self.resource_id,
                0,
                0, // x, y
                FB_WIDTH,
                FB_HEIGHT,
            ],
        );

        // Flush the display
        self.send_command(
            VIRTIO_GPU_CMD_RESOURCE_FLUSH,
            &[
                self.resource_id,
                0,
                0, // x, y
                FB_WIDTH,
                FB_HEIGHT,
            ],
        );
    }

    pub fn flush(&mut self) {
        self.update_display();
    }
}

unsafe impl Send for VirtIOGPU {}
unsafe impl Sync for VirtIOGPU {}

pub static GPU: Mutex<VirtIOGPU> = Mutex::new(VirtIOGPU::new());

pub fn init() {
    GPU.lock().init();
}
