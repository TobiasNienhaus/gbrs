use crate::game_boy::memory::MemError;
use crate::game_boy::video::PPU;
use std::path::PathBuf;

mod cpu;
pub mod memory;
mod video;

#[derive(Debug)]
pub enum GBRSError {
    MemError(memory::MemError),
}

impl From<memory::MemError> for GBRSError {
    fn from(e: MemError) -> Self {
        Self::MemError(e)
    }
}

// #[derive(Debug)]
pub struct GameBoy {
    cpu: cpu::Cpu,
}

impl GameBoy {
    const OAM_SEARCH_CLOCKS: u32 = 20;
    const PIXEL_TRANSFER_CLOCKS: u32 = 43;
    const H_BLANK_CLOCKS: u32 = 51;
    const CLOCKS_PER_LINE: u32 =
        GameBoy::OAM_SEARCH_CLOCKS + GameBoy::PIXEL_TRANSFER_CLOCKS + GameBoy::H_BLANK_CLOCKS;

    const DRAW_LINES: u32 = 144;
    const V_BLANK_LINES: u32 = 10;
    const LINES: u32 = GameBoy::DRAW_LINES + GameBoy::V_BLANK_LINES;

    const CLOCKS: u32 = GameBoy::LINES * GameBoy::CLOCKS_PER_LINE;

    pub fn load<'a>(path: &'_ PathBuf) -> Result<GameBoy, GBRSError> {
        let memory = memory::MMU::load_from_path(path)?;
        Ok(GameBoy {
            cpu: cpu::Cpu::new(memory),
        })
    }

    pub fn memory(&self) -> &memory::MMU {
        self.cpu.memory()
    }

    pub fn frame(&mut self, buffer: &mut Box<[u8]>) {
        let mut clocks_left = 0;
        for line in 0..GameBoy::LINES {
            self.cpu.memory_mut().set_ly(line as u8);
            if line < GameBoy::DRAW_LINES {
                for clock in 0..GameBoy::CLOCKS_PER_LINE {
                    // TODO set modes
                    if clock < GameBoy::OAM_SEARCH_CLOCKS {
                        // OAM search
                    } else if clock < GameBoy::PIXEL_TRANSFER_CLOCKS {
                        // Pixel transfer
                    } else {
                        // H-Blank
                    }
                    if clocks_left <= 0 {
                        clocks_left = self.cpu.tick();
                    } else {
                        clocks_left -= 1;
                    }
                    self.cpu.timer_clock_cycle();
                }
                PPU::write_line(self.cpu.memory_mut(), buffer);
            } else {
                for clock in 0..GameBoy::CLOCKS_PER_LINE {
                    // V-Blank
                    if clocks_left <= 0 {
                        clocks_left = self.cpu.tick();
                    } else {
                        clocks_left -= 1;
                    }
                    self.cpu.timer_clock_cycle();
                }
            }
        }
        // Per line:
        // - 20 clocks OAM search
        // - 43 clocks Pixel transfer
        // - 51 clocks H-Blank

        // In total:
        // - 144 lines
        // - 10 lines V-Blank
    }

    pub fn
}
