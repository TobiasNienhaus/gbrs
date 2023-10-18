use super::{interrupts::Interrupt, Cpu};

const CPU_CLOCK_SPEED: u32 = 1048576;
const MODE_00_TICKS: u32 = CPU_CLOCK_SPEED / 4096;
const MODE_01_TICKS: u32 = CPU_CLOCK_SPEED / 262144;
const MODE_10_TICKS: u32 = CPU_CLOCK_SPEED / 65536;
const MODE_11_TICKS: u32 = CPU_CLOCK_SPEED / 16384;

const DIVIDER_REGISTER: u16 = 0xFF04;
const TIMER_COUNTER: u16 = 0xFF05;
const TIMER_MODULO: u16 = 0xFF06;
const TIMER_CONTROL: u16 = 0xFF07;
const DIVIDER_CLOCKS: u32 = CPU_CLOCK_SPEED / 16384;

impl Cpu {
    fn timer_tick_count(&self) -> u32 {
        let tac = self.mmu.read_8(TIMER_CONTROL);
        match tac & 0b11 {
            0b00 => MODE_00_TICKS,
            0b01 => MODE_01_TICKS,
            0b10 => MODE_10_TICKS,
            0b11 => MODE_11_TICKS,
            _ => unreachable!(),
        }
    }

    fn timer_enabled(&self) -> bool {
        self.mmu.read_8(TIMER_CONTROL) & 0b100 == 0b100
    }

    fn timer_increase(&mut self) -> bool {
        // Read timer counter, increment it and reset it to timer modulo on overflow
        let tima = self.mmu.read_8(TIMER_COUNTER);
        let (mut result, overflow) = tima.overflowing_add(1);
        if overflow {
            result = self.mmu.read_8(TIMER_MODULO);
        }
        self.mmu.write_8(TIMER_COUNTER, result);
        return overflow;
    }

    fn divider_increase(&mut self) -> bool {
        // Read divider counter, increment it and reset it to timer modulo on overflow
        let tima = self.mmu.read_8(DIVIDER_REGISTER);
        let (result, overflow) = tima.overflowing_add(1);
        self.mmu.write_8(DIVIDER_REGISTER, result);
        return overflow;
    }

    pub fn timer_clock_cycle(&mut self) {
        // TODO timer interrupts
        let mut reset_divider = false;
        let mut reset_clock = false;
        // handle divider clock
        if self.clock_counter_divider % DIVIDER_CLOCKS == 0 {
            // Increase divider clock
            reset_divider = self.divider_increase();
            self.set_interrupt(Interrupt::TimerOverflow);
        }
        // handle time clock
        if self.clock_counter % self.timer_tick_count() == 0 && self.timer_enabled() {
            reset_clock = self.timer_increase();
            self.set_interrupt(Interrupt::TimerOverflow);
        }

        if reset_clock {
            self.clock_counter = 0;
        } else {
            self.clock_counter += 1;
        }
        if reset_divider {
            self.clock_counter_divider = 0;
        } else {
            self.clock_counter_divider += 1;
        }
    }
}
