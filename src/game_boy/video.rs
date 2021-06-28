use super::memory::MMU;

#[derive(Copy, Clone)]
pub enum VideoMode {
    HBlank,
    VBlank,
    OAM,
    PixelTransfer
}

pub struct PPU { }

impl PPU {
    pub fn render(mmu: &mut MMU, buffer: &mut Box<[u8]>) {

    }
}
