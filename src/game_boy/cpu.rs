mod instructions;
pub mod callmap;
pub use callmap::*;

use super::memory::MMU;

pub(super) struct Cpu {
    registers: [u8;8],
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
}

impl Cpu {
    pub fn new(mmu: MMU) -> Cpu {
        Cpu {
            registers: [0u8; 8],
            pc: 0x0100,
            sp: 0,
            mmu,
            interrupts_enabled: false,
            halted: false,
            stopped: false
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
    L
}

#[derive(Copy, Clone)]
enum Register16 {
    BC,
    DE,
    HL
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
            Register16::HL => (Register8::L, Register8::H)
        }
    }
}

pub enum Condition {
    ZSet,
    ZNotSet,
    CSet,
    CNotSet
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
    Vec8
}

#[derive(Copy, Clone)]
pub enum Interrupt {
    VBlank,
    LcdcStatus,
    TimerOverflow,
    SerialTransferCompletion,
    Input
}

impl Interrupt {
    /// The address to jump to when the interrupt occurs
    pub fn jump_address(&self) -> u16 {
        todo!()
    }

    /// The bit of the IF flag register for the specific interrupt
    pub fn if_bit(&self) -> u8 {
        todo!()
    }
}

fn check_bit(val: u8, bit: u8) -> bool {
    ((val >> bit) & 0x1) == 0x1
}

impl Cpu {
    const A_REG: usize = 0;
    const F_REG: usize = 1;
    const B_REG: usize = 2;
    const C_REG: usize = 3;
    const D_REG: usize = 4;
    const E_REG: usize = 5;
    const H_REG: usize = 6;
    const L_REG: usize = 7;

    fn reg(&self, register: Register8) -> u8 {
        match register {
            Register8::A => self.registers[Self::A_REG],
            Register8::F => self.registers[Self::F_REG],
            Register8::B => self.registers[Self::B_REG],
            Register8::C => self.registers[Self::C_REG],
            Register8::D => self.registers[Self::D_REG],
            Register8::E => self.registers[Self::E_REG],
            Register8::H => self.registers[Self::H_REG],
            Register8::L => self.registers[Self::L_REG],
        }
    }

    fn reg_mut(&mut self, register: Register8) -> &mut u8 {
        match register {
            Register8::A => &mut self.registers[Self::A_REG],
            Register8::F => &mut self.registers[Self::F_REG],
            Register8::B => &mut self.registers[Self::B_REG],
            Register8::C => &mut self.registers[Self::C_REG],
            Register8::D => &mut self.registers[Self::D_REG],
            Register8::E => &mut self.registers[Self::E_REG],
            Register8::H => &mut self.registers[Self::H_REG],
            Register8::L => &mut self.registers[Self::L_REG],
        }
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
            Condition::CNotSet => !self.carry_bit()
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
    fn read_u8(&mut self) -> u8 {
        let ret = self.mmu.read_8(self.pc);
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
        i8::from_le_bytes([self.read_u8()])
    }
}
