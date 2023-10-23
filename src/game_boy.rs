use crate::game_boy::memory::MemError;
use crate::game_boy::video::{PPU, VideoMode};
use std::path::PathBuf;
use crate::game_boy::cpu::debug::DebugStackInfo;
use crate::game_boy::cpu::interrupts::Interrupt;
use crate::game_boy::memory::video::LcdStatusBit;

pub mod cpu;
pub mod memory;
mod video;

#[derive(Debug)]
pub enum GBRSError {
    MemError(MemError),
}

impl From<MemError> for GBRSError {
    fn from(e: MemError) -> Self {
        Self::MemError(e)
    }
}

#[derive(Copy, Clone)]
pub struct InstructionInformation {
    instruction: u8,
    data: [Option<u8>; 4],
    stack_info: DebugStackInfo,
    is_new: bool,
    clocks_left: u32
}

impl InstructionInformation {
    pub fn instruction(&self) -> u8 { self.instruction }
    pub fn data(&self) -> [Option<u8>; 4] { self.data }
    pub fn stack_info(&self) -> DebugStackInfo { self.stack_info }
    pub fn is_new(&self) -> bool { self.is_new }
    pub fn clocks_left(&self) -> u32 { self.clocks_left }
}

#[derive(Copy, Clone)]
pub struct ClockInformation {
    instruction: InstructionInformation,
    frame_done: bool
}

impl ClockInformation {
    pub fn instruction(&self) -> InstructionInformation {
        self.instruction
    }

    pub fn frame_done(&self) -> bool {
        self.frame_done
    }

    pub fn new(instruction: u8, data: [Option<u8>; 4], stack_info: DebugStackInfo, is_new_instruction: bool, clocks_left: u32, frame_done: bool) -> ClockInformation {
        ClockInformation {
            instruction: InstructionInformation {
                instruction,
                data,
                stack_info,
                is_new: is_new_instruction,
                clocks_left
            },
            frame_done
        }
    }
}

// #[derive(Debug)]
pub struct GameBoy {
    cpu: cpu::Cpu,
    // cycles left in current instruction
    cycles_left_in_instruction: u32,
    clock_number_in_current_frame: u32,
    old_stat_interrupt_state: bool
}

// Constants
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

    const V_BLANK_INTERRUPT_CLOCK: u32 = Self::DRAW_LINES * Self::CLOCKS_PER_LINE;
}


// Initialization
impl GameBoy {
    pub fn load<'a>(path: &'_ PathBuf) -> Result<GameBoy, GBRSError> {
        let memory = memory::MMU::load_from_path(path)?;
        Ok(GameBoy {
            cpu: cpu::Cpu::new(memory),
            cycles_left_in_instruction: 0,
            clock_number_in_current_frame: 0,
            old_stat_interrupt_state: false
        })
    }
}

// Getters
impl GameBoy {
    pub fn memory(&self) -> &memory::MMU {
        self.cpu.memory()
    }

    pub fn cpu(&self) -> &cpu::Cpu {
        &self.cpu
    }
}

// Clocking
impl GameBoy {
    // Per line:
    // - 20 clocks OAM search
    // - 43 clocks Pixel transfer
    // - 51 clocks H-Blank

    // In total:
    // - 144 lines
    // - 10 lines V-Blank

    /// Do one single clock cycle in the GB-CPU
    pub fn clock(&mut self, buffer: &mut Box<[u8]>) -> bool {
        let ins = self.cpu.peek_instruction();
        let data = self.cpu.peek_data();
        let stack_info = self.cpu().debug_stack_info();
        let current_line = self.clock_number_in_current_frame / Self::CLOCKS_PER_LINE;
        let clock_in_line = self.clock_number_in_current_frame % Self::CLOCKS_PER_LINE;

        self.cpu.memory_mut().set_ly(current_line as u8);
        self.cpu.memory_mut().update_lyc_ly_cmp();

        let mut stat_interrupt_state = self.cpu.memory().get_lcd_status(LcdStatusBit::LycLyCmp);

        if self.clock_number_in_current_frame == Self::V_BLANK_INTERRUPT_CLOCK {
            self.cpu.set_interrupt(Interrupt::VBlank);
        }

        // TODO set modes
        if current_line < GameBoy::DRAW_LINES {
            if clock_in_line < GameBoy::OAM_SEARCH_CLOCKS {
                // OAM search
                self.cpu.memory_mut().set_video_mode(VideoMode::OAM);
                stat_interrupt_state |= self.cpu.memory().get_lcd_status(LcdStatusBit::OamStatInterrupt);
            } else if clock_in_line < GameBoy::PIXEL_TRANSFER_CLOCKS {
                // Pixel transfer
                self.cpu.memory_mut().set_video_mode(VideoMode::PixelTransfer);
            } else {
                // H-Blank
                self.cpu.memory_mut().set_video_mode(VideoMode::HBlank);
                stat_interrupt_state |= self.cpu.memory().get_lcd_status(LcdStatusBit::HBlankStatInterrupt);
            }
        } else {
            // V-Blank
            self.cpu.memory_mut().set_video_mode(VideoMode::VBlank);
            stat_interrupt_state |= self.cpu.memory().get_lcd_status(LcdStatusBit::VBlankStatInterrupt);
        }

        if !self.old_stat_interrupt_state && stat_interrupt_state {
            self.cpu.set_interrupt(Interrupt::LcdcStatus);
        }

        // TODO first draw or first new clock()
        let new_instruction = self._cpu_clock();

        if current_line < GameBoy::DRAW_LINES && clock_in_line + 1 == Self::CLOCKS_PER_LINE {
            PPU::write_line(self.cpu.memory_mut(), buffer);
        }

        self.clock_number_in_current_frame += 1;
        let new_frame = self.clock_number_in_current_frame == Self::CLOCKS;
        if new_frame { self.clock_number_in_current_frame = 0; }
        // ClockInformation::new(ins, data, stack_info, new_instruction, self.cycles_left_in_instruction, new_frame)
        new_frame
    }

    /// Try to do the next cpu instruction.
    /// Returns `true`, if a new instruction started
    /// Returns `false`, if the old one is still running
    fn _cpu_clock(&mut self) -> bool {
        let mut r = false;
        if self.cycles_left_in_instruction <= 0 {
            // tick() returns the time needed for the new instructions, _including_ the current
            // clock cycle -> tick() - 1
            self.cycles_left_in_instruction = self.cpu.tick() - 1;
            r = true;
        } else {
            self.cycles_left_in_instruction -= 1;
        }
        self.cpu.timer_clock_cycle();
        r
    }

    pub fn frame(&mut self, buffer: &mut Box<[u8]>) {
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
                    self._cpu_clock();
                }
                PPU::write_line(self.cpu.memory_mut(), buffer);
            } else {
                for _ in 0..GameBoy::CLOCKS_PER_LINE {
                    // V-Blank
                    self._cpu_clock();
                }
            }
        }

    }
}
