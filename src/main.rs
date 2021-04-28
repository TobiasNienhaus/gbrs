mod game_boy;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::path::PathBuf;
use std::str::FromStr;

// Actually 16742.005692282 microseconds
const REFRESH_RATE: u64 = 16742;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;
const PIXEL_COUNT: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

type FrameBuffer = [u32; PIXEL_COUNT];

fn main() {
    let gb = gameboy::GameBoy::load(&PathBuf::from_str("./dev/Tetris.gb").unwrap()).unwrap();

    println!("Title: {}", gb.meta_data().title());

    let mut buffer: FrameBuffer = [0; PIXEL_COUNT];
    for i in buffer.iter_mut() {
        *i = rand::random();
    }

    let mut window = Window::new(
        "GBRS",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.limit_update_rate(Some(std::time::Duration::from_micros(REFRESH_RATE)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            for i in buffer.iter_mut() {
                *i = rand::random();
            }
        }

        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
    }
}
