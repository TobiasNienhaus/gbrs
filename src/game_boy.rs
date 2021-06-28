use std::path::PathBuf;
use crate::game_boy::memory::MemError;

mod cpu;
mod memory;

#[derive(Debug)]
pub enum GBRSError {
    MemError(memory::MemError)
}

impl From<memory::MemError> for GBRSError {
    fn from(e: MemError) -> Self {
        Self::MemError(e)
    }
}

// #[derive(Debug)]
pub struct GameBoy {
    cpu: cpu::Cpu
}

impl GameBoy {
    const OAM_SEARCH_CLOCKS: u32 = 20;
    const PIXEL_TRANSFER_CLOCKS: u32 = 43;
    const H_BLANK_CLOCKS: u32 = 51;
    const CLOCKS_PER_LINE: u32 =
        GameBoy::OAM_SEARCH_CLOCKS +
        GameBoy::PIXEL_TRANSFER_CLOCKS +
        GameBoy::H_BLANK_CLOCKS;

    const DRAW_LINES: u32 = 144;
    const V_BLANK_LINES: u32 = 10;
    const LINES: u32 = GameBoy::DRAW_LINES + GameBoy::V_BLANK_LINES;

    const CLOCKS: u32 = GameBoy::LINES * GameBoy::CLOCKS_PER_LINE;

    pub fn load<'a>(path: &'_ PathBuf) -> Result<GameBoy, GBRSError> {
        let memory = memory::MMU::load_from_path(path)?;
        Ok(GameBoy {
            cpu: cpu::Cpu::new(memory)
        })
    }

    pub fn memory(&self) -> &memory::MMU {
        self.cpu.memory()
    }
    }
}
