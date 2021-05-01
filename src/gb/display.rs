use minifb::{self, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const SCALE: usize = 2;

pub struct Display {
    // NOCHECKIN: Remove this.
    #[allow(dead_code)]
    buffer: Vec<u32>,

    // NOCHECKIN: Remove this.
    #[allow(dead_code)]
    window: Window,
}

impl Display {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; 2 * SCALE * WIDTH * HEIGHT],
            window: Window::new(
                "boyo - Gameboy Emulator",
                WIDTH * SCALE,
                HEIGHT * SCALE,
                WindowOptions::default(),
            ).unwrap(), // NOCHECKIN: Error handle.
        }
    }

    pub fn draw(&mut self) {
        // NOCHECKIN: Error handle.
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
    }
}
