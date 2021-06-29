use super::memory::MMU;

pub const SCREEN_WIDTH: u8 = 160;

#[derive(Copy, Clone)]
pub enum VideoMode {
    HBlank,
    VBlank,
    OAM,
    PixelTransfer
}

pub struct PPU { }

impl PPU {
    pub fn write_line(mmu: &mut MMU, buffer: &mut Box<[u8]>) {
        // let line = mmu.read_ly();
        if mmu.background_enabled() {
            PPU::write_background_line(mmu, buffer);
        }

        if mmu.window_enabled() {

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
            let pixel_index = pixel_index as usize;
            buffer[pixel_index] = color;
        }
    }

    fn write_window_line(mmu: &mut MMU, buffer: &mut Box<[u8]>) {
        let palette = mmu.bg_palette();
        let tilemap = mmu.load_window_tilemap();

        let scroll_x = mmu.read_wx() - 7; // Because GB classic loves magic values
        let scroll_y = mmu.read_wy();

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
            let pixel_index = pixel_index as usize;
            buffer[pixel_index] = color;
        }
    }
}
