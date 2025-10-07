
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    /// Gameboy enters VBLANK
    VBlank,
    /// STAT Interrupts logically ORed
    LcdcStatus,
    /// Timer overflows (TIMA > 0xFF)
    TimerOverflow,
    /// Serial fuckery
    SerialTransferCompletion,
    /// P1 bits 0-3 change from HI to LOW -> Input happened
    Input,
}

impl Interrupt {
    /// The address to jump to when the interrupt occurs
    pub fn jump_address(&self) -> u16 {
        // If an interrupt occurs, push the PC to the stack and call the specified address
        match self {
            Interrupt::VBlank => 0x40,
            Interrupt::LcdcStatus => 0x48,
            Interrupt::TimerOverflow => 0x50,
            Interrupt::SerialTransferCompletion => 0x58,
            Interrupt::Input => 0x60,
        }
    }

    /// The bit of the IF flag register for the specific interrupt
    pub fn if_ie_bit(&self) -> u8 {
        match self {
            Interrupt::VBlank => 0,
            Interrupt::LcdcStatus => 1,
            Interrupt::TimerOverflow => 2,
            Interrupt::SerialTransferCompletion => 3,
            Interrupt::Input => 4,
        }
    }
}