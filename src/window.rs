// 59.727500569606 Hz
// Actually 16742.706298828 microseconds
const REFRESH_RATE: u64 = 16743;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

use minifb::{Key, Window, WindowOptions};

// According to https://en.wikipedia.org/wiki/Game_Boy#Technical_specifications
const COLOR_LOOKUP: [u32; 4] = [0xFF0F380F, 0xFF306230, 0xFF8BAC0F, 0xFF9BBC0F];

pub struct GbWindow {
    true_width: usize,
    true_height: usize,
    magnification: usize,
    small_buffer: Box<[u8]>,
    big_buffer: Box<[u32]>,
    window: Window,
}

impl GbWindow {
    pub fn new(magnification: usize) -> GbWindow {
        let mut window = Window::new(
            "GBRS",
            SCREEN_WIDTH * magnification,
            SCREEN_HEIGHT * magnification,
            WindowOptions::default(),
        )
        .unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(REFRESH_RATE)));
        GbWindow {
            true_width: SCREEN_WIDTH * magnification,
            true_height: SCREEN_HEIGHT * magnification,
            magnification,
            small_buffer: vec![0; PIXEL_COUNT].into_boxed_slice(),
            big_buffer: vec![0; PIXEL_COUNT * magnification * magnification].into_boxed_slice(),
            window,
        }
    }

    pub fn buffer_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.small_buffer
    }

    pub const fn buffer_size() -> usize {
        PIXEL_COUNT
    }

    fn big_buffer_size(&self) -> usize {
        self.true_width * self.true_height
    }

    pub fn display(&mut self) {
        assert_eq!(
            Self::buffer_size() * self.magnification * self.magnification,
            self.big_buffer_size()
        );
        // Very slow this thing
        for i_small in 0..Self::buffer_size() {
            let val = self.small_buffer[i_small];
            let y = (i_small / SCREEN_WIDTH) * self.magnification;
            let x = (i_small % SCREEN_WIDTH) * self.magnification;

            for y_it in y..(y + self.magnification) {
                for x_it in x..(x + self.magnification) {
                    let i_big = y_it * self.true_width + x_it;
                    self.big_buffer[i_big] = COLOR_LOOKUP[val as usize];
                }
            }
        }
        self.window
            .update_with_buffer(&self.big_buffer, self.true_width, self.true_height)
            .unwrap();
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn win(&self) -> &Window {
        &self.window
    }
}
