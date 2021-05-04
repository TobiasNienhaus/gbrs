use crate::game_boy::memory::rom::RomError;
use std::path::PathBuf;

pub mod rom;

#[derive(Debug)]
pub enum MemError {
    RomError(rom::RomError)
}

impl From<rom::RomError> for MemError {
    fn from(e: RomError) -> Self {
        MemError::RomError(e)
    }
}

const MEM_SIZE: usize = 0xFFFF;

/// The memory region
pub(super) enum MemRegion {
    /// External Bus ROM Region
    Rom,
    /// VRAM -> duh
    VRam,
    /// External Bus RAM Region
    Ram,
    /// No idea... yet
    WRam,
    /// WRAM, just echoed for some reason
    Echo,
    /// Object Attribute Memory
    Oam,
    /// Object Attribute Memory, but hear me out... invalid
    InvalidOam,
    /// Memory mapped I/O
    IOMemMap,
    /// High RAM
    HRam,
    /// IE register
    IEReg
}

impl MemRegion {
    pub fn get_region(address: u16) -> MemRegion {
        match address {
            0x0..=0x7FFF => MemRegion::Rom,
            0x8000..=0x9FFF => MemRegion::VRam,
            0xA000..=0xBFFF => MemRegion::Ram,
            0xC000..=0xDFFF => MemRegion::WRam,
            0xE000..=0xFDFF => MemRegion::Echo,
            0xFE00..=0xFE9F => MemRegion::Oam,
            0xFEA0..=0xFEFF => MemRegion::InvalidOam,
            0xFF00..=0xFF7F => MemRegion::IOMemMap,
            0xFF80..=0xFFFE => MemRegion::HRam,
            0xFFFF => MemRegion::IEReg,
            _ => unreachable!("You somehow called a function with a u16 outside of the range of a u16. Congration you done it!")
        }
    }

    pub const fn get_region_start(region: MemRegion) -> u16 {
        match region {
            MemRegion::Rom => 0x0,
            MemRegion::VRam => 0x8000,
            MemRegion::Ram => 0xA000,
            MemRegion::WRam => 0xC000,
            MemRegion::Echo => 0xE000,
            MemRegion::Oam => 0xFE00,
            MemRegion::InvalidOam => 0xFEA0,
            MemRegion::IOMemMap => 0xFF00,
            MemRegion::HRam => 0xFF80,
            MemRegion::IEReg => 0xFFFF
        }
    }

    pub fn get_offset_in_region(address: u16) -> u16 {
        address - Self::get_region_start(Self::get_region(address))
    }
}

#[derive(Debug)]
pub struct Memory {
    // For now put it on the stack :^) -> it SHOULD be able to handle 64kiB
    mem: [u8; MEM_SIZE],
    rom: rom::Rom
}

impl Memory {
    pub fn load_from_path(path: &PathBuf) -> Result<Memory, MemError> {
        let rom = rom::Rom::load_from_path(path)?;
        Ok(Memory {
            mem: [0; MEM_SIZE],
            rom
        })
    }

    pub fn rom(&self) -> &rom::Rom {
        &self.rom
    }
}