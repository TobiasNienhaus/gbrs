use crate::game_boy::interrupt::Interrupt;
use super::Cpu;

impl Cpu {
    /// Request an interrupt, meaning it will possibly be handled next -> set the specific bit of IF high
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        self.mmu.request_interrupt(interrupt);
    }

    /// Reset a requested interrupt, meaning it has been handled -> set the specific bit of IF low
    fn reset_requested_interrupt(&mut self, interrupt: Interrupt) {
        self.mmu.reset_requested_interrupt(interrupt);
    }

    /// check if a specific interrupt is enabled in the IE-flags
    fn interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        self.mmu.interrupt_enabled(interrupt)
    }

    fn interrupt_requested(&self, interrupt: Interrupt) -> bool {
        self.mmu.interrupt_requested(interrupt)
    }

    fn execute_interrupt(&mut self, interrupt: Interrupt) -> bool {
        // TODO keine Ahnung, ob das so passt
        if self.interrupt_requested(interrupt) {
            println!("Interrupted: {:?}", interrupt);
            // IME disabled
            self.interrupts_enabled = false;
            // IF disabled
            self.reset_requested_interrupt(interrupt);
            // Regular call to jump address
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
