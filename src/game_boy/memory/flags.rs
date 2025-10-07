
mod interrupts {
    use crate::game_boy::helpers::check_bit;
    use crate::game_boy::interrupt::Interrupt;
    use crate::game_boy::memory::{addresses as adr, MMU};

    impl MMU {
        pub fn request_interrupt(&mut self, interrupt: Interrupt) {
            let if_val = self.read_8(adr::interrupts::FLAGS) | (1 << interrupt.if_ie_bit());
            self.write_8(adr::interrupts::FLAGS, if_val);
        }

        /// Reset a requested interrupt, meaning it has been handled -> set the specific bit of IF low
        pub fn reset_requested_interrupt(&mut self, interrupt: Interrupt) {
            let if_val = self.read_8(adr::interrupts::FLAGS) & !(1 << interrupt.if_ie_bit());
            self.write_8(adr::interrupts::FLAGS, if_val);
        }

        /// check if a specific interrupt is enabled in the IE-flags
        pub fn interrupt_enabled(&self, interrupt: Interrupt) -> bool {
            check_bit(
                self.read_8(adr::interrupts::ENABLE),
                interrupt.if_ie_bit(),
            )
        }

        pub fn interrupt_requested(&self, interrupt: Interrupt) -> bool {
            self.interrupt_enabled(interrupt)
                && check_bit(self.read_8(adr::interrupts::FLAGS), interrupt.if_ie_bit())
        }
    }
}