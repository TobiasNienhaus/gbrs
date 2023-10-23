/// Interrupt-addresses
pub mod interrupts {
    pub const ENABLE: u16 = 0xFFFF;
    pub const FLAGS: u16 = 0xFF0F;
}

/// Timers-addresses
pub mod timer {
    pub const DIVIDER_REGISTER: u16 = 0xFF04;
    pub const COUNTER: u16 = 0xFF05;
    pub const MODULO: u16 = 0xFF06;
    pub const CONTROL: u16 = 0xFF07;
}

/// Video-adresses
pub mod video {
    pub const SCREEN_Y: u16 = 0xFF42;
    pub const SCREEN_X: u16 = 0xFF43;
    /// The Y position of the window area.
    pub const WINDOW_Y: u16 = 0xFF4A;
    /// The X position of the window area. (For some reason minus 7)
    pub const WINDOW_X: u16 = 0xFF4B;
    /// Current line being drawn -> LY
    pub const CURRENT_LINE: u16 = 0xFF44;
    /// 1. The Game Boy constantly compares the value of the LYC and LY registers. When both values are identical, the “LYC=LY” flag in the STAT register is set, and (if enabled) a STAT interrupt is requested.
    ///
    /// 2. Compared to `LY`. If they are similar, the `STAT` register is set and (if enabled)
    /// an interrupt is sent
    ///
    /// -> LYC
    pub const LINE_COMPARE: u16 = 0xFF45;
    pub const LCD_STATUS: u16 = 0xFF41;

    pub const LCD_CONTROL: u16 = 0xFF40;
}

pub mod memory {
    pub const BOOT_ROM_ENABLED: u16 = 0xFF50;
}
