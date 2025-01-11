use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq)]
pub enum WindowState {
    Normal,
    Minimizing(f32), // Animation progress 0.0 to 1.0
    Minimized,
    Maximizing(f32), // Animation progress 0.0 to 1.0
    Maximized,
}

pub struct WindowManager {
    windows: Vec<Window>,
    current_desktop: usize,
    desktops: Vec<Vec<usize>>, // Window indices for each desktop
    focused_window: Option<usize>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            current_desktop: 0,
            desktops: vec![Vec::new()],
            focused_window: None,
        }
    }

    pub fn add_desktop(&mut self) {
        self.desktops.push(Vec::new());
    }

    pub fn switch_desktop(&mut self, index: usize) {
        if index < self.desktops.len() {
            self.current_desktop = index;
        }
    }

    pub fn update_animations(&mut self) {
        for window in &mut self.windows {
            match window.state {
                WindowState::Minimizing(progress) => {
                    if progress < 1.0 {
                        window.state = WindowState::Minimizing(progress + 0.1);
                    } else {
                        window.state = WindowState::Minimized;
                    }
                }
                WindowState::Maximizing(progress) => {
                    if progress < 1.0 {
                        window.state = WindowState::Maximizing(progress + 0.1);
                    } else {
                        window.state = WindowState::Maximized;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self) {
        let mut gpu = GPU.lock();

        // Draw only windows on current desktop
        for &window_index in &self.desktops[self.current_desktop] {
            let window = &self.windows[window_index];

            match window.state {
                WindowState::Normal | WindowState::Maximized => {
                    window.draw();
                }
                WindowState::Minimizing(progress) | WindowState::Maximizing(progress) => {
                    window.draw_animated(progress);
                }
                WindowState::Minimized => {}
            }
        }
    }
}
