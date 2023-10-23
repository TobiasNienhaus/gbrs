pub mod callmap;
mod instructions;
pub use callmap::*;
pub mod interrupts;
pub mod time;
pub mod debug;

use super::memory::MMU;

pub struct Cpu {
    registers: [u8; 8],
    // a_reg: u8, // Accumulator
    // flag_reg: u8,
    // b_reg: u8,
    // c_reg: u8,
    // d_reg: u8,
    // e_reg: u8,
    // h_reg: u8,
    // l_reg: u8,
    pc: u16,
    sp: u16,
    mmu: MMU,
    interrupts_enabled: bool,
    halted: bool,
    stopped: bool,
    clock_counter_divider: u32,
    clock_counter: u32,
}

impl Cpu {
    pub fn new(mmu: MMU) -> Cpu {
        Cpu {
            registers: [0u8; 8],
            pc: 0x0, // 0x0100,
            sp: 0xFFFE,
            mmu,
            interrupts_enabled: false,
            halted: false,
            stopped: false,
            clock_counter_divider: 0,
            clock_counter: 0,
        }
    }

    pub fn memory(&self) -> &MMU {
        &self.mmu
    }

    pub fn memory_mut(&mut self) -> &mut MMU {
        &mut self.mmu
    }
}

#[derive(Copy, Clone)]
enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Register8 {
    pub fn idx(&self) -> usize {
        match self {
            Register8::A => 0,
            Register8::F => 1,
            Register8::B => 2,
            Register8::C => 3,
            Register8::D => 4,
            Register8::E => 5,
            Register8::H => 6,
            Register8::L => 7,
        }
    }
}

#[derive(Copy, Clone)]
enum Register16 {
    BC,
    DE,
    HL,
}

impl Register16 {
    /// Split the 16 bit register into the low 8 bit register and the high 8 bit register.
    ///
    /// This will return a tuple of 8 bit register in this form:
    /// `(low: Register8, high: Register8)`.
    fn split(&self) -> (Register8, Register8) {
        match self {
            Register16::BC => (Register8::C, Register8::B),
            Register16::DE => (Register8::E, Register8::D),
            Register16::HL => (Register8::L, Register8::H),
        }
    }
}

pub enum Condition {
    ZSet,
    ZNotSet,
    CSet,
    CNotSet,
}

#[derive(Copy, Clone)]
pub enum ResetVec {
    // TODO better naming, maybe...
    Vec1,
    Vec2,
    Vec3,
    Vec4,
    Vec5,
    Vec6,
    Vec7,
    Vec8,
}

fn check_bit(val: u8, bit: u8) -> bool {
    ((val >> bit) & 0b1) == 0b1
}

impl Cpu {
    fn reg(&self, register: Register8) -> u8 {
        self.registers[register.idx()]
    }

    fn reg_mut(&mut self, register: Register8) -> &mut u8 {
        &mut self.registers[register.idx()]
    }

    fn a_reg(&self) -> u8 {
        self.reg(Register8::A)
    }

    fn a_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::A)
    }

    fn f_reg(&self) -> u8 {
        self.reg(Register8::F)
    }

    fn f_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::F)
    }

    fn b_reg(&self) -> u8 {
        self.reg(Register8::B)
    }

    fn b_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::B)
    }

    fn c_reg(&self) -> u8 {
        self.reg(Register8::C)
    }

    fn c_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::C)
    }

    fn d_reg(&self) -> u8 {
        self.reg(Register8::D)
    }

    fn d_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::D)
    }

    fn e_reg(&self) -> u8 {
        self.reg(Register8::E)
    }

    fn e_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::E)
    }

    fn h_reg(&self) -> u8 {
        self.reg(Register8::H)
    }

    fn h_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::H)
    }

    fn l_reg(&self) -> u8 {
        self.reg(Register8::L)
    }

    fn l_reg_mut(&mut self) -> &mut u8 {
        self.reg_mut(Register8::L)
    }

    fn write_reg16(&mut self, reg: Register16, val: u16) {
        let (low, high) = reg.split();
        let bytes = val.to_le_bytes();
        *self.reg_mut(low) = bytes[0];
        *self.reg_mut(high) = bytes[1];
    }

    fn reg16(&self, reg: Register16) -> u16 {
        let (low, high) = reg.split();
        // TODO check if byte order is correct
        u16::from_le_bytes([self.reg(low), self.reg(high)])
    }

    pub fn dump_flags(&self) {
        println!(
            "ZERO: {}\nHALF CARRY: {}\nCARRY: {}\nNEGATIVE: {}",
            self.zero_bit(),
            self.half_carry_bit(),
            self.carry_bit(),
            self.negative_bit()
        )
    }
}

impl Cpu {
    fn set_flag_bit(&mut self, bit: u8, high: bool) {
        if high {
            *self.f_reg_mut() |= 1 << bit;
        } else {
            *self.f_reg_mut() &= !(1 << bit);
        }
    }

    fn flag_bit(&self, bit: u8) -> bool {
        ((self.f_reg() >> bit) & 0x1) == 0x1
    }

    pub(super) fn zero_bit(&self) -> bool {
        self.flag_bit(7)
    }

    pub(super) fn set_zero_bit(&mut self, high: bool) {
        self.set_flag_bit(7, high);
    }

    pub(super) fn carry_bit(&self) -> bool {
        self.flag_bit(4)
    }

    pub(super) fn set_carry_bit(&mut self, high: bool) {
        self.set_flag_bit(4, high);
    }

    pub(super) fn half_carry_bit(&self) -> bool {
        self.flag_bit(5)
    }

    pub(super) fn set_half_carry_bit(&mut self, high: bool) {
        self.set_flag_bit(5, high);
    }

    pub(super) fn negative_bit(&self) -> bool {
        self.flag_bit(6)
    }

    pub(super) fn set_negative_bit(&mut self, high: bool) {
        self.set_flag_bit(6, high);
    }

    pub(super) fn check_condition(&self, cond: Condition) -> bool {
        match cond {
            Condition::ZSet => self.zero_bit(),
            Condition::ZNotSet => !self.zero_bit(),
            Condition::CSet => self.carry_bit(),
            Condition::CNotSet => !self.carry_bit(),
        }
    }

    pub(super) fn set_flags_from_byte(&mut self, byte: u8) {
        // TODO is this correct?
        *self.f_reg_mut() = (self.f_reg() & 0xF) | (byte & 0xF0);
    }

    fn is_running(&self) -> bool {
        !self.halted && !self.stopped
    }
}

impl Cpu {
    fn peek_u8(&self) -> u8 {
        self.mmu.read_8(self.pc)
    }

    fn read_u8(&mut self) -> u8 {
        let ret = self.peek_u8();
        self.pc += 1;
        ret
    }

    fn read_u16(&mut self) -> u16 {
        // TODO don't unwrap
        let ret = self.mmu.read_16(self.pc).unwrap();
        self.pc += 2;
        ret
    }

    fn read_i8(&mut self) -> i8 {
        // TODO is the i8 read correctly?
        i8::from_le_bytes([self.read_u8()])
    }
}
