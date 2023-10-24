#![feature(generators, generator_trait)]

mod game_boy;
mod window;

use minifb::{Key, KeyRepeat};
use std::str::FromStr;
use std::time::Instant;

use crate::window::GbWindow;
use clap::{crate_version, App, Arg};
use rand::Rng;
use crate::game_boy::cpu::debug::ins_name;

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
// https://github.com/Baekalfen/PyBoy/blob/master/PyBoy.pdf
// https://gbdev.io/pandocs
// https://gbdev.io
// https://gbdev.io/gb-opcodes/optables/ <- Opcodes
// https://github.com/Rodrigodd/gameroy#resources-to-be-thankful-for
// https://gekkio.fi/files/gb-docs/gbctr.pdf

// TODO
// in 0xFFB8 werden falsche Daten geladen
// bei Schritt 15989 ist PC falsch. IST: 0233 SOLL: 0239
// Der Conditional Jump aus Schritt 15988 sollte eigentlicht NICHT springen
// -> Eine Flag ist falsch

struct CliOpts {
    rom_path: String,
    magnification: usize,
}

impl CliOpts {
    fn load() -> CliOpts {
        let matches = App::new("GB-rs")
            .version(crate_version!())
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
    // println!(
    //     "Mem region: {:?}",
    //     game_boy::memory::MemRegion::get_region(0xC3C8)
    // );
    let opts = CliOpts::load();
    let mut gb = game_boy::GameBoy::load(&opts.rom_path.into()).unwrap();
    //
    // gb.memory().rom().print_meta();

    let mut window = GbWindow::new(opts.magnification);

    // for i in window.buffer_mut().iter_mut() {
    //     *i = rand::thread_rng().gen_range(0..=3);
    // }

    let mut run = true;

    let mut cou = 0;

    while window.is_open() {
        let frame_start = Instant::now();
        let mut has_clocks_left_in_frame = true;

        while has_clocks_left_in_frame {
            let info = gb.clock(window.buffer_mut());
            has_clocks_left_in_frame = !info.frame_done();
            if info.instruction().is_new() {
                // println!(
                //     "[{:#06X}] [{:02X}] {{{}}} -> {}",
                //     info.instruction().pc(),
                //     info.instruction().instruction(),
                //     format!(
                //         "{} {} {} {}",
                //         if let Some(byte) = info.instruction().data()[0] { format!("{:02X}", byte) } else { "NN".to_owned() },
                //         if let Some(byte) = info.instruction().data()[1] { format!("{:02X}", byte) } else { "NN".to_owned() },
                //         if let Some(byte) = info.instruction().data()[2] { format!("{:02X}", byte) } else { "NN".to_owned() },
                //         if let Some(byte) = info.instruction().data()[3] { format!("{:02X}", byte) } else { "NN".to_owned() },
                //     ),
                //     ins_name(info.instruction().instruction())
                // );
                if !gb.cpu().memory().boot_rom_enabled() {
                // if info.instruction().stack_info().pc() >= 0x100 {
                    println!(
                        "{:04X} {:02X} {} {}",
                        info.instruction().stack_info().pc(),
                        info.instruction().instruction(),
                        info.instruction().stack_info(),
                        format!(
                            "{} {} {} {}",
                            if let Some(byte) = info.instruction().data()[0] { format!("{:02X}", byte) } else { "NN".to_owned() },
                            if let Some(byte) = info.instruction().data()[1] { format!("{:02X}", byte) } else { "NN".to_owned() },
                            if let Some(byte) = info.instruction().data()[2] { format!("{:02X}", byte) } else { "NN".to_owned() },
                            if let Some(byte) = info.instruction().data()[3] { format!("{:02X}", byte) } else { "NN".to_owned() },
                        ),
                    );
                }
            }
            // run = has_clocks_left_in_frame;
        }
        // eprintln!("Frame! {}", cou);

        cou += 1;
        let before_display = frame_start.elapsed().as_nanos();
        // for i in window.buffer_mut().iter_mut() {
        //     *i = rand::thread_rng().gen_range(0..=3);
        // }
        window.display();
        let end = frame_start.elapsed().as_nanos();
        let display_time = end - before_display;
        // println!(
        //     "{} ({}) | {} ({}) | {}",
        //     before_display,
        //     before_display as f64 / end as f64,
        //     display_time,
        //     display_time as f64 / end as f64,
        //     end
        // );
        //
        // if cou % 250 == 0 {
        //     eprintln!("Frame {}", cou)
        // }
    }
}

// GENERAL TODO
// TODO Writing to Divider Register sets it to 0
// TODO Interrupts
