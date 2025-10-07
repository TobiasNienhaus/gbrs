use super::memory::MMU;

pub const SCREEN_WIDTH: u8 = 160;

#[derive(Copy, Clone)]
pub enum VideoMode {
    HBlank,
    VBlank,
    OAM,
    PixelTransfer
}

#[derive(Copy, Clone)]
struct FifoPixel(u8);

impl From<u8> for FifoPixel {
    fn from(value: u8) -> Self {
        FifoPixel(value)
    }
}

impl FifoPixel {
    fn color(&self) -> u8 {
        self.0 & 0b00000011
    }

    fn palette(&self) -> u8 {
        self.0 & 0b00011100
    }

    // TODO bool???
    fn bg_priority(&self) -> bool {
        todo!()
    }
}

type FIFO = [FifoPixel; 8];

pub struct PPU {
    dot_delay: u8,
    fifo_bg: FIFO,
    fifo_obj: FIFO,
    counter_bg: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            dot_delay: 0,
            fifo_bg: [0.into(); 8],
            fifo_obj: [0.into(); 8],
            counter_bg: 1
        }
    }

    /// Iterate a single dot in the PPU (normally 4 per 1 CPU clock)
    pub fn dot(&mut self, mmu: &mut MMU, buffer: &mut Box<[u8]>) {
        if self.counter_bg == 1 {} else if self.counter_bg == 3 {} else if self.counter_bg == 5 {}
        if self.counter_bg >= 9 && self.push(mmu, buffer) {
            self.counter_bg = 0;
        } else {
            self.counter_bg += 1;
        }
    }

    fn push(&mut self, mmu: &mut MMU, buffer: &mut Box<[u8]>) -> bool {
        todo!("Push pixels onto the fifo_bg")
    }
}

// old
impl PPU {
    pub fn write_line(mmu: &mut MMU, buffer: &mut Box<[u8]>) {
        // let line = mmu.read_ly();
        if mmu.background_enabled() {
            PPU::write_background_line(mmu, buffer);
        }

        if mmu.window_enabled() && mmu.background_enabled() {
            PPU::write_window_line(mmu, buffer);
        }
    }

    fn write_background_line(mmu: &mut MMU, buffer: &mut Box<[u8]>) {
        let palette = mmu.bg_palette();
        let tilemap = mmu.load_bg_tilemap();

        let scroll_x = mmu.read_scx();
        let scroll_y = mmu.read_scy();

        let y = mmu.read_ly();

        for x in 0..SCREEN_WIDTH {
            let scrolled_x = x.overflowing_add(scroll_x).0;
            let scrolled_y = y.overflowing_add(scroll_y).0;

            let tile_index = (scrolled_x / 8, scrolled_y / 8); // The location of the sprite in the tilemap
            let tile_pixel = (scrolled_x % 8, scrolled_y % 8); // The pixel of the sprite to draw

            const TILE_BYTE_SIZE: u8 = 16; // For now just make it a constant (Tile not implemented yet)

            let sprite = mmu.read_sprite(
                tilemap.get_tile_address(tile_index.0 as usize, tile_index.1 as usize)
            );
            let color = sprite.get(tile_pixel.0 as usize, tile_pixel.1 as usize);
            let color = palette.remap(color);
            let pixel_index = y as usize * SCREEN_WIDTH as usize + x as usize;
            let pixel_index = pixel_index;
            buffer[pixel_index] = color;
        }
    }

    fn write_window_line(mmu: &mut MMU, buffer: &mut Box<[u8]>) {
        let palette = mmu.bg_palette();
        let tilemap = mmu.load_window_tilemap();

        // let scroll_x = mmu.read_wx() - 7; // Because GB classic loves magic values
        // let scroll_y = mmu.read_wy();

        let scroll_x = mmu.read_scx();
        let scroll_y = mmu.read_scy();

        let y = mmu.read_ly();

        for x in 0..SCREEN_WIDTH {
            let scrolled_x = x.overflowing_add(scroll_x).0;
            let scrolled_y = y.overflowing_add(scroll_y).0;

            let tile_index = (scrolled_x / 8, scrolled_y / 8); // The location of the sprite in the tilemap
            let tile_pixel = (scrolled_x % 8, scrolled_y % 8); // The pixel of the sprite to draw

            // const TILE_BYTE_SIZE: u8 = 16; // For now just make it a constant (Tile not implemented yet)

            let sprite = mmu.read_sprite(
                tilemap.get_tile_address(tile_index.0 as usize, tile_index.1 as usize)
            );
            let color = sprite.get(tile_pixel.0 as usize, tile_pixel.1 as usize);
            let color = palette.remap(color);
            let pixel_index = y as usize * SCREEN_WIDTH as usize + x as usize;
            let pixel_index = pixel_index as usize;
            buffer[pixel_index] = color;
        }
    }

    fn mixed_fifo(&self) -> FIFO {
        todo!()
    }
}
