use std::path::PathBuf;
use std::fs::File;
use std::io::{SeekFrom, Read, Seek};
use std::convert::TryInto;

#[allow(unused)]
const NINTENDO_GRAPHIC: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug)]
pub enum RomError {
    IoError(std::io::Error),
    UnsupportedRomSize,
    UnsupportedRamSize,
    UnsupportedCartridgeType(u8),
    InvalidRomData
}

impl From<std::io::Error> for RomError {
    fn from(e: std::io::Error) -> RomError {
        RomError::IoError(e)
    }
}

pub type RomResult<T> = Result<T, RomError>;

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly,
    // TODO
}

impl CartridgeType {
    fn from_byte(b: u8) -> RomResult<CartridgeType> {
        Ok(match b {
            0x0 => CartridgeType::RomOnly,
            _ => return Err(RomError::UnsupportedCartridgeType(b)),
        })
    }
}

#[derive(Debug)]
pub struct Rom {
    // Meta data
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
    /// The underlying data.
    /// The size is signified by rom_size
    data: Box<[u8]>
}

// Getters
impl Rom {
    /// The minimum length of the supplied array for loading from bytes.
    /// This is basically the size of the smallest ROM size the GB had.
    /// Anything below that is invalid.
    const MIN_SUPPLIED_BYTE_ARRAY_LEN: usize = 32_768;
    /// The title of the ROM
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Does the ROM contain a GB Color game?
    pub fn is_color(&self) -> bool {
        self.color
    }

    /// Does the game support Super GameBoy functions?
    pub fn is_super_game_boy(&self) -> bool {
        self.super_game_boy
    }

    /// Is the game meant for a japanese market?
    pub fn is_japanese(&self) -> bool {
            self.japanese
        }
}

fn rom_size_from_byte(byte: u8) -> RomResult<usize> {
    Ok(match byte {
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
        _ => return Err(RomError::UnsupportedRomSize),
    })
}

fn ram_size_from_byte(byte: u8) -> RomResult<usize> {
    Ok(match byte {
        0x0 => 0,
        0x1 => 2_048,
        0x2 => 8_192,
        0x3 => 32_768,
        0x4 => 131_072,
        _ => return Err(RomError::UnsupportedRamSize),
    })
}

fn title_from_bytes(bytes: &[u8]) -> String {
    let mut title = String::with_capacity(16);
    for b in bytes.iter().take_while(|b| **b != 0x0) {
        title.push(*b as char);
    }
    title
}

// Public functions
impl Rom {
    const ROM_SIZE_LOC: usize = 0x148;
    const RAM_SIZE_LOC: usize = 0x149;

    const IS_SUPER_GAME_BOY_LOC: usize = 0x146;
    const CARTRIDGE_TYPE_LOC: usize = 0x147;
    const JAPANESE_ROM_LOC: usize = 0x14A;
    const IS_COLOR_LOC: usize = 0x143;

    const TITLE_LOC_RANGE: std::ops::RangeInclusive<usize> = 0x134..=0x142;

    pub fn load_from_path(path: &PathBuf) -> RomResult<Rom> {
        let mut file = File::open(path)?;
        let byte_count = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;
        let mut temp_buffer = Vec::with_capacity(byte_count as usize);
        file.read_to_end(&mut temp_buffer)?;
        Rom::load_from_bytes(temp_buffer.into_boxed_slice())
    }

    pub fn load_from_bytes(bytes: Box<[u8]>) -> RomResult<Rom> {
        if bytes.len() < Rom::MIN_SUPPLIED_BYTE_ARRAY_LEN {
            return Err(RomError::InvalidRomData);
        }
        let rom_size = rom_size_from_byte(bytes[Rom::RAM_SIZE_LOC])?;
        if bytes.len() < rom_size {
            return Err(RomError::InvalidRomData);
        }
        Ok(Rom {
            title: title_from_bytes(&bytes[Rom::TITLE_LOC_RANGE]),
            color: bytes[Rom::IS_COLOR_LOC] == 0x80,
            rom_size,
            ram_size: ram_size_from_byte(bytes[Rom::RAM_SIZE_LOC])?,
            super_game_boy: bytes[Rom::IS_SUPER_GAME_BOY_LOC] == 0x03,
            cartridge_type: CartridgeType::from_byte(bytes[Rom::CARTRIDGE_TYPE_LOC])?,
            japanese: bytes[Rom::JAPANESE_ROM_LOC] == 0x0,
            data: bytes
        })
    }

    // TODO handle banks, etc.
    pub fn read_8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    // TODO handle banks, etc.
    pub fn read_16(&self, address: u16) -> u16 {
        // Doesn't handle any overflows, etc.
        let a = address as usize;
        u16::from_le_bytes(self.data[a..a+1].try_into().unwrap())
    }
    // TODO write methods??? Don't make sense with ROM
}

// Debug functions
impl Rom {
    pub fn print_meta(&self) {
        println!("TITLE: {}", self.title);
        println!("ROM SIZE: {}", self.rom_size);
        println!("RAM SIZE: {}", self.ram_size);
        println!("CARTRIDGE TYPE: {:?}", self.cartridge_type);
        println!("IS COLOR: {}", self.color);
        println!("SUPPORTS SUPER GB: {}", self.super_game_boy);
        println!("JAPANESE MARKET: {}", self.japanese);
    }
}
