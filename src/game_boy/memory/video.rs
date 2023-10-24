use super::*;
use super::super::video::VideoMode;

use addresses as adr;

impl VideoMode {
    fn to_two_bit(&self) -> u8 {
        match self {
            VideoMode::HBlank => 0b00,
            VideoMode::VBlank => 0b01,
            VideoMode::OAM => 0b10,
            VideoMode::PixelTransfer => 0b11,
        }
    }

    fn from_two_bit(val: u8) -> VideoMode {
        match val & 0b11 {
            0b00 => VideoMode::HBlank,
            0b01 => VideoMode::VBlank,
            0b10 => VideoMode::OAM,
            0b11 => VideoMode::PixelTransfer,
            _ => unreachable!("Somehow the bitwise and didn't do shit")
        }
    }
}

fn check_bit(byte: u8, bit: u8) -> bool {
    (byte >> bit) & 0x1 == 0x1
}

pub enum SpriteSize {
    Single,
    Double
}

#[derive(Copy, Clone)]
pub enum BgTileMapMode {
    Signed,
    Unsigned
}

impl BgTileMapMode {
    pub fn address_range(&self) -> std::ops::RangeInclusive<u16> {
        match self {
            BgTileMapMode::Signed => 0x8800..=0x97FF,
            BgTileMapMode::Unsigned => 0x8000..=0x8FFF,
        }
    }

    pub fn pivot(&self) -> u16 {
        match self {
            BgTileMapMode::Signed => 0x9000, // 0x8800, // TODO it should center around 0x9000, right?
            BgTileMapMode::Unsigned => 0x8000,
        }
    }
}

pub struct LcdcSettings {
    display_enabled: bool,
    window_tile_map_display_select: SelectedTileMap,
    window_enabled: bool,
    // Indices of tiles
    bg_and_window_tile_data_select: BgTileMapMode,
    bg_tile_map_display_select: SelectedTileMap,
    sprite_size: SpriteSize,
    sprite_display_enabled: bool,
    background_enabled: bool
}

pub enum SelectedTileMap {
    High,
    Low
}

pub enum LcdStatusBit {
    LycStatInterrupt,
    OamStatInterrupt,
    VBlankStatInterrupt,
    HBlankStatInterrupt,
    LycLyCmp
}

impl LcdStatusBit {
    fn bit(&self) -> u8 {
        match self {
            Self::LycStatInterrupt => 6,
            Self::OamStatInterrupt => 5,
            Self::VBlankStatInterrupt => 4,
            Self::HBlankStatInterrupt => 3,
            Self::LycLyCmp => 2,
        }
    }
}

pub enum LcdControlBit {
    LcdPpuEnable,
    WindowTileMapArea,
    WindowEnable,
    BgWindowTileDataArea,
    BgTileMapArea,
    ObjSize,
    ObjEnable,
    BgWindowEnable
}

impl LcdControlBit {
    fn bit(&self) -> u8 {
        match self {
            LcdControlBit::LcdPpuEnable => 7,
            LcdControlBit::WindowTileMapArea => 6,
            LcdControlBit::WindowEnable => 5,
            LcdControlBit::BgWindowTileDataArea => 4,
            LcdControlBit::BgTileMapArea => 3,
            LcdControlBit::ObjSize => 2,
            LcdControlBit::ObjEnable => 1,
            LcdControlBit::BgWindowEnable => 0,
        }
    }
}

impl SelectedTileMap {
    pub fn start_address(&self) -> u16 {
        match self {
            SelectedTileMap::High => 0x9C00,
            SelectedTileMap::Low => 0x9800,
        }
    }
}

impl LcdcSettings {
    fn from_byte(byte: u8) -> LcdcSettings {
        LcdcSettings {
            display_enabled: check_bit(byte, LcdControlBit::LcdPpuEnable.bit()),
            window_tile_map_display_select: if check_bit(byte, LcdControlBit::WindowTileMapArea.bit()) {
                SelectedTileMap::High
            } else {
                SelectedTileMap::Low
            },
            window_enabled: check_bit(byte, LcdControlBit::WindowEnable.bit()),
            bg_and_window_tile_data_select: if check_bit(byte, LcdControlBit::BgWindowTileDataArea.bit()) {
                BgTileMapMode::Unsigned
            } else {
                BgTileMapMode::Signed
            },
            bg_tile_map_display_select: if check_bit(byte, 3) {
                SelectedTileMap::High
            } else {
                SelectedTileMap::Low
            },
            sprite_size: if check_bit(byte, 2) { SpriteSize::Double } else { SpriteSize::Single },
            sprite_display_enabled: check_bit(byte, 1),
            background_enabled: check_bit(byte, 0)
        }
    }
}

impl MMU {
    pub fn read_lcdc_settings(&self) -> LcdcSettings {
        LcdcSettings::from_byte(self.read_8(adr::video::LCD_CONTROL))
    }

    pub fn display_enabled(&self) -> bool {
        self.get_lcdc_bit(7)
    }

    pub fn window_tile_map_display_select(&self) -> SelectedTileMap {
        if self.get_lcdc_bit(6) {
            SelectedTileMap::High
        } else {
            SelectedTileMap::Low
        }
    }

    pub fn window_enabled(&self) -> bool {
        self.get_lcdc_bit(5)
    }

    pub fn bg_and_window_tile_data_select(&self) -> BgTileMapMode {
        if self.get_lcdc_bit(4) {
            BgTileMapMode::Unsigned
        } else {
            BgTileMapMode::Signed
        }
    }

    pub fn bg_tile_map_display_select(&self) -> SelectedTileMap {
        if self.get_lcdc_bit(3) {
            SelectedTileMap::High
        } else {
            SelectedTileMap::Low
        }
    }

    pub fn sprite_size(&self) -> SpriteSize {
        if self.get_lcdc_bit(2) {
            SpriteSize::Double
        } else {
            SpriteSize::Single
        }
    }

    pub fn sprite_display_enabled(&self) -> bool {
        self.get_lcdc_bit(1)
    }

    pub fn background_enabled(&self) -> bool {
        self.get_lcdc_bit(0)
    }

    fn set_lcdc_bit(&mut self, bit: u8, high: bool) {
        let mut val = self.read_8(adr::video::LCD_CONTROL);
        if high {
            val |= 1 << bit;
        } else {
            val &= !(1 << bit);
        }
        self.write_8(adr::video::LCD_CONTROL, val);
    }

    fn get_lcdc_bit(&self, bit: u8) -> bool {
        let val = self.read_8(adr::video::LCD_CONTROL);
        (val >> bit) & 0x1 == 0x1
    }
}

// LCD Control

// LCD Status
impl MMU {
    pub fn enable_lyc_interrupt(&mut self, high: bool) {
        self.set_stat_bit(6, high);
    }

    pub fn get_lyc_interrupt_enabled(&self) -> bool {
        self.get_stat_bit(6)
    }

    pub fn enable_oam_interrupt(&mut self, high: bool) {
        self.set_stat_bit(5, high);
    }

    pub fn get_oam_interrupt_enabled(&self) -> bool {
        self.get_stat_bit(5)
    }

    pub fn enable_vblank_interrupt(&mut self, high: bool) {
        self.set_stat_bit(4, high);
    }

    pub fn get_vblank_interrupt_enabled(&self) -> bool {
        self.get_stat_bit(4)
    }

    pub fn enable_hblank_interrupt(&mut self, high: bool) {
        self.set_stat_bit(3, high);
    }

    pub fn get_hblank_interrupt_enabled(&self) -> bool {
        self.get_stat_bit(3)
    }

    // pub fn set_coincidence_flag(&mut self, high: bool) {
    //     self.set_stat_bit(2, high);
    // }
    //
    // pub fn get_coincidence_flag(&self) -> bool {
    //     self.get_stat_bit(2)
    // }

    pub fn update_lyc_ly_cmp(&mut self) {
        self.set_lcd_status(LcdStatusBit::LycLyCmp, self.read_lyc() == self.read_ly());
    }

    pub fn set_lcd_status(&mut self, bit: LcdStatusBit, high: bool) {
        self.set_stat_bit(bit.bit(), high);
    }

    pub fn get_lcd_status(&self, bit: LcdStatusBit) -> bool {
        self.get_stat_bit(bit.bit())
    }

    pub fn set_video_mode(&mut self, mode: VideoMode) {
        let mut val = self.read_8(adr::video::LCD_STATUS);
        val = (val & 0xFC) | mode.to_two_bit();
        self.write_8(adr::video::LCD_STATUS, val);
    }

    pub fn get_video_mode(&self) -> VideoMode {
        VideoMode::from_two_bit(self.read_8(adr::video::LCD_STATUS) & 0b11)
    }

    fn set_stat_bit(&mut self, bit: u8, high: bool) {
        let mut val = self.read_8(adr::video::LCD_STATUS);
        if high {
            val |= 1 << bit;
        } else {
            val &= !(1 << bit);
        }
        self.write_8(adr::video::LCD_STATUS, val);
    }

    fn get_stat_bit(&self, bit: u8) -> bool {
        let val = self.read_8(adr::video::LCD_STATUS);
        (val >> bit) & 0x1 == 0x1
    }
}

impl MMU {
    pub fn set_scy(&mut self, val: u8) {
        self.write_8(adr::video::SCREEN_Y, val);
    }

    pub fn read_scy(&self) -> u8 {
        self.read_8(adr::video::SCREEN_Y)
    }

    pub fn set_scx(&mut self, val: u8) {
        self.write_8(adr::video::SCREEN_X, val);
    }

    pub fn read_scx(&self) -> u8 {
        self.read_8(adr::video::SCREEN_X)
    }

    pub fn set_ly(&mut self, val: u8) {
        self.write_8(adr::video::CURRENT_LINE, val);
    }

    pub fn read_ly(&self) -> u8 {
        self.read_8(adr::video::CURRENT_LINE)
    }

    pub fn set_lyc(&mut self, val: u8) {
        self.write_8(adr::video::LINE_COMPARE, val);
    }

    pub fn read_lyc(&self) -> u8 {
        self.read_8(adr::video::LINE_COMPARE)
    }

    pub fn set_wy(&mut self, val: u8) {
        self.write_8(adr::video::WINDOW_Y, val);
    }

    pub fn read_wy(&self) -> u8 {
        self.read_8(adr::video::WINDOW_Y)
    }

    pub fn set_wx(&mut self, val: u8) {
        self.write_8(adr::video::WINDOW_X, val);
    }

    pub fn read_wx(&self) -> u8 {
        self.read_8(adr::video::WINDOW_X)
    }
}

pub struct Palette {
    mapping: [u8; 4],
}

impl Palette {
    fn from_byte(byte: u8) -> Palette {
        // 0b11000000             -> 6 shifts
        // 0b00110000 | +2 shifts -> 4 shifts
        // 0b00001100 | +2 shifts -> 2 shifts
        // 0b00000011 | +2 shifts -> 0 shifts
        Palette {
            mapping: [
                byte & 0b11,
                (byte >> 2) & 0b11,
                (byte >> 4) & 0b11,
                (byte >> 6) & 0b11
            ]
        }
    }

    pub fn remap(&self, color: u8) -> u8 {
        self.mapping[(color & 0b11) as usize]
    }
}

impl MMU {
    const BG_PALETTE: u16 = 0xFF47;

    pub fn bg_palette(&self) -> Palette {
        Palette::from_byte(self.read_8(MMU::BG_PALETTE))
    }

    const SPRITE_PALETTE_0: u16 = 0xFF48;

    pub fn sprite_palette_0(&self) -> Palette {
        Palette::from_byte(self.read_8(MMU::SPRITE_PALETTE_0))
    }

    const SPRITE_PALETTE_1: u16 = 0xFF49;

    pub fn sprite_palette_1(&self) -> Palette {
        Palette::from_byte(self.read_8(MMU::SPRITE_PALETTE_1))
    }
}

pub struct Sprite {
    colors: [[u8; 8]; 8]
}

impl Sprite {
    /// A tile is 16 bytes, with 2 bits per color
    fn from_u128(bytes: u128) -> Sprite {
        // TODO is this at all correct?
        let mut colors = [[0u8; 8]; 8];
        for (idx, b) in bytes.to_le_bytes().iter().enumerate() {
            let line = idx / 2;
            let low_bit = idx % 2 == 0; // Or is it 1?
            for bit in 0..8 {
                if low_bit {
                    colors[line][bit] |= (b >> bit) & 0b1;
                } else {
                    colors[line][bit] |= ((b >> bit) & 0b1) << 1;
                }
            }
        }
        Sprite {
            colors
        }
    }

    fn from_bytes(bytes: [u8; 16]) -> Sprite {
        Sprite::from_u128(u128::from_le_bytes(bytes))
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        // Lines first, then rows
        self.colors[y][x]
    }
}

impl MMU {
    pub fn read_sprite(&self, address: u16) -> Sprite {
        Sprite::from_u128(self.read_128(address))
    }
}

#[derive(Copy, Clone)]
pub enum TileMapTile {
    Signed(i8),
    Unsigned(u8)
}

impl TileMapTile {
    pub fn unsigned_from_byte(byte: u8) -> TileMapTile {
        TileMapTile::Unsigned(byte)
    }

    pub fn signed_from_byte(byte: u8) -> TileMapTile {
        TileMapTile::Signed(i8::from_le_bytes([byte]))
    }

    pub fn unwrap_signed(self) -> i8 {
        match self {
            TileMapTile::Signed(num) => num,
            TileMapTile::Unsigned(_) => panic!("Unwrapping unsigned TileMapTile as signed!"),
        }
    }

    pub fn unwrap_unsigned(self) -> u8 {
        match self {
            TileMapTile::Unsigned(num) => num,
            TileMapTile::Signed(_) => panic!("Unwrapping unsigned TileMapTile as signed!"),
        }
    }
}

pub struct BgTileMap {
    mode: BgTileMapMode,
    map: [[TileMapTile; 32]; 32]
}

impl BgTileMap {
    pub fn get_tile_address(&self, x: usize, y: usize) -> u16 {
        match self.mode {
            BgTileMapMode::Signed => {
                // TODO figure out, if this wraps correctly => -1 -> u16::MAX - 1
                ((self.mode.pivot() as i32) + (self.map[y][x].unwrap_signed() as i32)) as u16
            }
            BgTileMapMode::Unsigned => {
                self.mode.pivot().overflowing_add(self.map[y][x].unwrap_unsigned() as u16).0
            }
        }
    }
}

impl MMU {
    pub fn load_bg_tilemap(&self) -> BgTileMap {
        let mode = self.bg_and_window_tile_data_select();
        let selected = self.bg_tile_map_display_select();
        self.load_tilemap(selected.start_address())
    }

    pub fn load_window_tilemap(&self) -> BgTileMap {
        let selected = self.window_tile_map_display_select();
        self.load_tilemap(selected.start_address())
    }

    fn load_tilemap(&self, address: u16) -> BgTileMap {
        let mode = self.bg_and_window_tile_data_select();
        let mut map = [[TileMapTile::Unsigned(0); 32]; 32];
        for idx in 0..32 * 32 {
            let x = idx % 32;
            let y = idx / 32;
            let byte = self.read_8(address + idx);
            map[y as usize][x as usize] = match mode {
                BgTileMapMode::Signed => TileMapTile::signed_from_byte(byte),
                BgTileMapMode::Unsigned => TileMapTile::unsigned_from_byte(byte),
            }
        }
        BgTileMap {
            mode,
            map
        }
    }
}
