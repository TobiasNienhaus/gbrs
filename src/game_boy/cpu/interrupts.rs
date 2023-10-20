use super::{super::memory::adresses as adr, check_bit, Cpu};

#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    VBlank,
    LcdcStatus,
    TimerOverflow,
    SerialTransferCompletion,
    Input,
}

impl Interrupt {
    /// The address to jump to when the interrupt occurs
    pub fn jump_address(&self) -> u16 {
        // TODO keine Ahnung, was ich hier mache >:D
        match self {
            Interrupt::VBlank => 0x40,
            Interrupt::LcdcStatus => 0x48,
            Interrupt::TimerOverflow => 0x58,
            Interrupt::SerialTransferCompletion => 0x50,
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

impl Cpu {
    pub fn set_interrupt(&mut self, interrupt: Interrupt) {
        let if_val = self.mmu.read_8(adr::INTERRUPT_FLAGS) | (1 << interrupt.if_ie_bit());
        self.mmu.write_8(adr::INTERRUPT_FLAGS, if_val);
    }

    fn unset_interrupt(&mut self, interrupt: Interrupt) {
        let if_val = self.mmu.read_8(adr::INTERRUPT_FLAGS) & !(1 << interrupt.if_ie_bit());
        self.mmu.write_8(adr::INTERRUPT_FLAGS, if_val);
    }

    fn interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        check_bit(
            self.mmu.read_8(adr::INTERRUPT_ENABLE),
            interrupt.if_ie_bit(),
        )
    }

    fn interrupt_requested(&self, interrupt: Interrupt) -> bool {
        self.interrupt_enabled(interrupt)
            && check_bit(self.mmu.read_8(adr::INTERRUPT_FLAGS), interrupt.if_ie_bit())
    }

    fn execute_interrupt(&mut self, interrupt: Interrupt) -> bool {
        // TODO keine Ahnung, ob das so passt
        if self.interrupt_requested(interrupt) {
            println!("Interrupted: {:?}", interrupt);
            self.interrupts_enabled = false;
            self.unset_interrupt(interrupt);
            self.call(interrupt.jump_address());
            return true;
        }
        false
    }

    // Handles all interrupts in order of priority. Returns true, if one was handled
    //
    // If one was handled, the GB waits for 5 cycles
    pub fn handle_interrupts(&mut self) -> bool {
        // TODO https://gbdev.io/pandocs/Interrupts.html#interrupt-handling
        if !self.interrupts_enabled {
            return false;
        }
        if self.execute_interrupt(Interrupt::VBlank) {
            return true;
        }
        if self.execute_interrupt(Interrupt::LcdcStatus) {
            return true;
        }
        if self.execute_interrupt(Interrupt::TimerOverflow) {
            return true;
        }
        if self.execute_interrupt(Interrupt::SerialTransferCompletion) {
            return true;
        }
        if self.execute_interrupt(Interrupt::Input) {
            return true;
        }
        false
    }
}
