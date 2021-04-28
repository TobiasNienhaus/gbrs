use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

// Max rom size in bytes
// const MAX_ROM_SIZE: usize = 2_048_000;
// Im pretty sure its the bottom one -> 16 MebiBytes
const MAX_ROM_SIZE: usize = 2_097_152;
type Rom = Box<[u8]>;

const NINTENDO_GRAPHIC: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug)]
enum CartridgeType {
    RomOnly,
    Unsupported, // TODO
}

impl CartridgeType {
    pub fn from_byte(b: u8) -> CartridgeType {
        match b {
            0x0 => CartridgeType::RomOnly,
            _ => CartridgeType::Unsupported,
        }
    }
}

#[derive(Debug)]
pub struct RomData {
    /// Game title
    title: String,
    /// Game Boy color or not
    color: bool,
    /// Rom size in bytes
    rom_size: usize,
    /// Ram size in bytes
    ram_size: usize,
    /// Are Super GameBoy functions supported?
    super_game_boy: bool,
    /// The cartridge type
    cartridge_type: CartridgeType,
    /// Is the destination japanese?
    japanese: bool,
}

impl RomData {
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn is_color(&self) -> bool {
        self.color
    }

    fn rom_size(&self) -> usize {
        self.rom_size
    }

    fn ram_size(&self) -> usize {
        self.ram_size
    }

    pub fn is_super_game_boy(&self) -> bool {
        self.super_game_boy
    }

    pub fn is_japanese(&self) -> bool {
        self.japanese
    }
}

impl RomData {
    fn read_rom_size(b: u8) -> usize {
        match b {
            // TODO are these correct?
            0x0 => 32_768,
            0x1 => 65_536,
            0x2 => 131_072,
            0x3 => 262_144,
            0x4 => 524_288,
            0x5 => 1_048_576,
            0x6 => 2_097_152,
            0x52 => 1_179_648,
            0x53 => 1_310_720,
            0x54 => 1_572_864,
            _ => 0,
        }
    }

    fn read_ram_size(b: u8) -> usize {
        match b {
            0x0 => 0,
            0x1 => 2_048,
            0x2 => 8_192,
            0x3 => 32_768,
            0x4 => 131_072,
            _ => 0,
        }
    }

    fn from_rom(rom: &Rom) -> RomData {
        let bytes = &rom[0x134..=0x142];
        let mut title = String::with_capacity(16);
        for b in bytes.iter().take_while(|b| **b != 0x0) {
            title.push(*b as char);
        }
        RomData {
            title,
            color: rom[0x143] == 0x80,
            rom_size: Self::read_rom_size(rom[0x148]),
            ram_size: Self::read_ram_size(rom[0x149]),
            super_game_boy: rom[0x146] == 0x03,
            cartridge_type: CartridgeType::from_byte(rom[0x147]),
            japanese: rom[0x14A] == 0x0,
        }
    }
}

pub struct GameBoy {
    rom_meta_data: RomData,
    rom: Rom,
}

#[derive(Debug)]
pub enum LoadingError {
    IoError(std::io::Error),
    UnsupportedRom,
}

impl From<std::io::Error> for LoadingError {
    fn from(e: std::io::Error) -> LoadingError {
        LoadingError::IoError(e)
    }
}

pub type LoadingResult<T> = Result<T, LoadingError>;

fn load_rom(path: &PathBuf) -> LoadingResult<Rom> {
    let mut file = File::open(path)?;
    let byte_count = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0));
    let mut temp_buffer = Vec::with_capacity(byte_count as usize);
    file.read_to_end(&mut temp_buffer)?;
    Ok(temp_buffer.into_boxed_slice())
}

impl GameBoy {
    pub fn load(path: &PathBuf) -> LoadingResult<GameBoy> {
        let rom = load_rom(path)?;

        let rom_meta_data = RomData::from_rom(&rom);

        Ok(GameBoy {
            rom_meta_data,
            // Does that actually work?
            rom,
        })
    }

    pub fn meta_data(&self) -> &RomData {
        &self.rom_meta_data
    }
}
