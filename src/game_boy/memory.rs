use rom::RomError;
use std::convert::TryInto;
use std::ops::Range;
use std::path::PathBuf;

pub mod addresses;
pub mod misc;
pub mod rom;
pub mod video;

use addresses as adr;

const BOOT_ROM: &[u8] = include_bytes!("../../res/boot/dmg_boot.bin");

#[derive(Debug)]
pub enum MemError {
    RomError(rom::RomError),
    InvalidAddressRegion(MemRegion),
    Invalid2ByteAccess,
    OutOfBounds,
    BootRomOutOfBounds
}

pub type MemResult<T> = Result<T, MemError>;

impl From<rom::RomError> for MemError {
    fn from(e: RomError) -> Self {
        MemError::RomError(e)
    }
}

const MEM_SIZE: usize = 0x10000;
const NON_ROM_SIZE: usize = 0x10000 - 0x8000;

/// The memory region
#[derive(Debug, Copy, Clone)]
pub enum MemRegion {
    /// External Bus ROM Region
    Rom,
    /// VRAM -> duh
    VRam,
    /// External Bus RAM Region
    Ram,
    /// No idea... yet
    /// I think it's the internal RAM
    WRam,
    /// WRAM, just echoed for some reason
    /// Can be seen as empty
    Echo,
    /// Object Attribute Memory
    /// Special purpose VRAM
    /// Basically META for Sprites
    Oam,
    /// Object Attribute Memory, but hear me out... invalid
    InvalidOam,
    /// Memory mapped I/O
    IOMemMap,
    /// High RAM
    HRam,
    /// IE register
    IEReg,
}

impl MemRegion {
    pub fn get_region(address: u16) -> MemRegion {
        match address {
            0x0..=0x7FFF => MemRegion::Rom,
            0x8000..=0x9FFF => MemRegion::VRam,
            0xA000..=0xBFFF => MemRegion::Ram, // External RAM?
            0xC000..=0xDFFF => MemRegion::WRam,
            0xE000..=0xFDFF => MemRegion::Echo, // Prohibited
            0xFE00..=0xFE9F => MemRegion::Oam,
            0xFEA0..=0xFEFF => MemRegion::InvalidOam, // Prohibited
            0xFF00..=0xFF7F => MemRegion::IOMemMap,
            0xFF80..=0xFFFE => MemRegion::HRam,
            u16::MAX => MemRegion::IEReg, // 0xFFFF is not supported by IntelliJ Rust extension
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
            MemRegion::IEReg => 0xFFFF,
        }
    }

    pub fn is_region_end(address: u16) -> bool {
        match address {
            0x7FFF | 0x9FFF | 0xBFFF | 0xDFFF | 0xFDFF | 0xFE9F | 0xFEFF | 0xFF7F | 0xFFFE => true,
            _ => false,
        }
    }

    pub fn get_offset_in_region(address: u16) -> u16 {
        address - Self::get_region_start(Self::get_region(address))
    }

    pub fn is_in_boot_rom(address: u16) -> bool {
        address < 0x100
    }
}

#[derive(Debug)]
pub struct MMU {
    // For now put it on the stack :^) -> it SHOULD be able to handle 64kiB
    mem: [u8; NON_ROM_SIZE],
    rom: rom::Rom,
}

// const DBG_ADDRESS: &[u16] = &[0xFEu16, adr::video::LCD_CONTROL, adr::memory::BOOT_ROM_ENABLED];
const DBG_WRITE_ADDRESSES: &[u16] = &[0xFF50u16];
const DBG_READ_ADDRESSES: &[u16] = &[0xFF];

impl MMU {
    const ROM_REGION: Range<u16> = 0x0..0x8000;

    // TODO handle boot ROM
    pub fn load_from_path(path: &PathBuf) -> MemResult<MMU> {
        let rom = rom::Rom::load_from_path(path)?;
        Ok(MMU {
            mem: [0; NON_ROM_SIZE],
            rom,
        })
    }

    pub fn rom(&self) -> &rom::Rom {
        &self.rom
    }

    // TODO let read_8 return MemResult

    pub fn read_8(&self, address: u16) -> u8 {
        if self.boot_rom_enabled() && MemRegion::is_in_boot_rom(address) {
            self.read_8_boot(address)
        } else if MMU::ROM_REGION.contains(&address) {
            self.rom.read_8(address)
        } else {
            self.mem[address as usize - 0x8000]
        }
    }

    fn read_8_boot(&self, address: u16) -> u8 {
        if MemRegion::is_in_boot_rom(address) {
            BOOT_ROM[address as usize]
        } else {
            0xFF
        }
    }

    // TODO maybe just return bool?


    pub fn write_8(&mut self, address: u16, val: u8) -> MemResult<()> {
        // TODO there are some special addresses with specific behavior
        // 0xFF46 -> Transfer ROM or RAM to OAM
        if address == MMU::DMA {
            self.dma_transfer(val);
            Ok(())
        } else if MMU::ROM_REGION.contains(&address) {
            Err(MemError::InvalidAddressRegion(MemRegion::get_region(
                address,
            )))
        } else {
            self.mem[address as usize - 0x8000] = val;
            Ok(())
        }
    }

    pub fn read_16(&self, address: u16) -> MemResult<u16> {
        // Do 2-byte reads have to be aligned to a 2-byte grid?
        // If yes a simple modulo is enough
        // TODO handle 2 Byte read on 0xFF when Boot ROM enabled
        if MemRegion::is_region_end(address) {
            Err(MemError::Invalid2ByteAccess)
        } else {
            if MemRegion::is_in_boot_rom(address) && self.boot_rom_enabled() {
                let a = address.into();
                Ok(u16::from_le_bytes(BOOT_ROM[a..a + 2].try_into().unwrap()))
            } else if MMU::ROM_REGION.contains(&address) {
                Ok(self.rom.read_16(address))
            } else if address as usize - 0x8000 >= NON_ROM_SIZE - 1 {
                Err(MemError::OutOfBounds)
            } else {
                let a = address as usize - 0x8000;
                Ok(u16::from_le_bytes(self.mem[a..a + 2].try_into().unwrap()))
            }
        }
    }

    fn read_16_boot(&self, address: u16) -> MemResult<u16> {
        if address >= 0x100 - 2 {
            Err(MemError::BootRomOutOfBounds)
        } else {
            let a: usize = address.into();
            Ok(u16::from_le_bytes(BOOT_ROM[a..a + 2].try_into().unwrap()))
        }
    }

    pub fn write_16(&mut self, address: u16, val: u16) -> MemResult<()> {
        if MemRegion::is_region_end(address) {
            Err(MemError::Invalid2ByteAccess)
        } else if MMU::ROM_REGION.contains(&address) {
            Err(MemError::InvalidAddressRegion(MemRegion::get_region(
                address,
            )))
        } else {
            let bytes = val.to_le_bytes();
            self.mem[address as usize] = bytes[0];
            self.mem[address as usize + 1] = bytes[1];
            Ok(())
        }
    }

    pub fn read_128(&self, address: u16) -> u128 {
        if MMU::ROM_REGION.contains(&address) {
            todo!()
        } else {
            let address = (address as usize) - 0x8000; // Subtract ROM region
            u128::from_le_bytes(
                self.mem[address as usize..address as usize + 16]
                    .try_into()
                    .unwrap(),
            )
        }
        // TODO correctly handle errors, etc. (over boundary write, etc.)
        // Read the next 16 bytes from memory and put them in a u128
        // TODO allow Boot ROM Reads
    }

    pub fn boot_rom_enabled(&self) -> bool {
        self.mem[adr::memory::BOOT_ROM_ENABLED as usize - 0x8000] == 0x00
    }
}
