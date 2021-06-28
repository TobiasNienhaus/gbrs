mod game_boy;
mod window;

use minifb::{Key, KeyRepeat};
use std::str::FromStr;

use crate::window::GbWindow;
use clap::{App, Arg};
use rand::Rng;

// Links:
// Endianness Guide:
// -> https://pastebin.com/5BEvWb2h
// -> GB classic is Little Endian
// https://gbdev.gg8.se/wiki/articles/Main_Page
// https://mgba-emu.github.io/gbdoc/
// https://rgbds.gbdev.io/docs/v0.5.0/gbz80.7
// https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
// http://gameboy.mongenel.com/dmg/asmmemmap.html
// http://bgb.bircd.org/pandocs.htm
// https://github.com/gbdev/awesome-gbdev
// https://ladecadence.net/trastero/listado%20juegos%20gameboy.html
// https://romhustler.org/roms/gbc/number
// https://github.com/aidan-clyens/GBExperience

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
            .unwrap_or(2);
        CliOpts {
            rom_path,
            magnification,
        }
    }
}

fn main() {
    let opts = CliOpts::load();
    let mut gb = game_boy::GameBoy::load(&opts.rom_path.into()).unwrap();

    gb.memory().rom().print_meta();

    let mut window = GbWindow::new(opts.magnification);

    for i in window.buffer_mut().iter_mut() {
        *i = rand::thread_rng().gen_range(0..=3);
    }

    while window.is_open() {
        // if window.win().is_key_pressed(Key::Space, KeyRepeat::No) {
            for i in window.buffer_mut().iter_mut() {
                *i = rand::thread_rng().gen_range(0..=3);
            }
        // }

        gb.frame();

        window.display();
    }
}
