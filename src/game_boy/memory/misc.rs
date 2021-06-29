use super::*;

impl MMU {
    pub(super) const DMA: u16 = 0xFF46;

    pub(super) fn dma_transfer(&mut self, index: u8) {
        // TODO somehow only allow access to HRAM during DMA transfer
        let source = index as u16 * 0x100;
        const DEST: u16 = 0xFE00;
        for i in 0..=0xF1 {
            let read = self.read_8(source + i);
            self.write_8(DEST + i);
        }
    }
}