mod game_boy;
mod window;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::path::PathBuf;
use std::str::FromStr;

use crate::window::GbWindow;
use clap::{App, Arg};

struct CliOpts {
    rom_path: String,
    magnification: usize,
}

impl CliOpts {
    fn load() -> CliOpts {
        let matches = App::new("GB-rs")
            .arg(Arg::with_name("rom-path").required(true).index(1))
            .arg(
                Arg::with_name("magnification")
                    .short("m")
                    .long("magnification")
                    .value_name("VAL"),
            )
            .get_matches();
        let rom_path = matches.value_of("rom-path").unwrap().to_owned();
        let magnification = matches
            .value_of("magnification")
            .map(|o| usize::from_str(o).expect("Could not parse number"))
            .unwrap_or(1);
        CliOpts {
            rom_path,
            magnification,
        }
    }
}

fn main() {
    let opts = CliOpts::load();
    let gb = game_boy::GameBoy::load(&opts.rom_path.into()).unwrap();

    println!("Title: {}", gb.meta_data().title());

    let mut window = GbWindow::new(opts.magnification);

    for (idx, i) in window.buffer_mut().iter_mut().enumerate() {
        let v = ((idx % 10) as f32 / 10f32) * u8::MAX as f32;
        *i = (rand::random::<f32>() * v) as u32;
    }

    while window.is_open() {
        if window.win().is_key_pressed(Key::Space, KeyRepeat::No) {
            for i in window.buffer_mut().iter_mut() {
                *i = rand::random();
            }
        }

        window.display();
    }
}
