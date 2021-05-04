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

#[derive(Debug)]
pub struct GameBoy {
    memory: memory::MMU
}

impl GameBoy {
    pub fn load(path: &PathBuf) -> Result<GameBoy, GBRSError> {
        let memory = memory::MMU::load_from_path(path)?;
        Ok(GameBoy {
            memory
        })
    }

    pub fn memory(&self) -> &memory::MMU {
        &self.memory
    }
}
