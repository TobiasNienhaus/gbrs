
use super::memory::MMU;

pub(super) struct Cpu<'a> {
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
    mmu: &'a mut MMU
    // There's another 8-bit register -> the location HL point to
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

impl Cpu<'_> {
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

    fn reg_16(&self, register: Register16) -> u16 {
        match register {
            Register16::BC => u16::from_le_bytes([self.b_reg(), self.c_reg()]),
            Register16::DE => u16::from_le_bytes([self.d_reg(), self.e_reg()]),
            Register16::HL => u16::from_le_bytes([self.h_reg(), self.l_reg()])
        }
    }

    fn write_reg_16(&mut self, val: u16, register: Register16) {
        match register {
            Register16::BC => self.write_bc(val),
            Register16::DE => self.write_de(val),
            Register16::HL => self.write_hl(val)
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

    pub(super) fn write_af(&mut self, val: u16) {
        let af = val.to_le_bytes();
        *self.a_reg_mut() = af[0];
        // prevent writing bits 0-3
        *self.f_reg_mut() = (af[1] & 0xF0) | (self.f_reg() & 0x0F);
    }

    pub(super) fn write_bc(&mut self, val: u16) {
        let bc = val.to_le_bytes();
        *self.b_reg_mut() = bc[0];
        *self.c_reg_mut() = bc[1];
    }

    pub(super) fn write_de(&mut self, val: u16) {
        let de = val.to_le_bytes();
        *self.d_reg_mut() = de[0];
        *self.e_reg_mut() = de[1];
    }

    pub(super) fn write_hl(&mut self, val: u16) {
        let hl = val.to_le_bytes();
        *self.h_reg_mut() = hl[0];
        *self.l_reg_mut() = hl[1];
    }

    fn read_hl(&self) -> u16 {
        u16::from_le_bytes([self.h_reg(), self.l_reg()])
    }
}

impl Cpu<'_> {
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
}

impl Cpu<'_> {
    /// Add the value in <reg> to A plus the carry flag
    ///
    /// Takes 1 cycle
    fn adc_reg(&mut self, reg: Register8) {
        self.adc(self.reg(reg));
    }

    /// Add the value that HL points to to A plus the carry flag
    ///
    /// 2 cycles
    fn adc_hl(&mut self) {
        self.adc(self.mmu.read_8(self.read_hl()))
    }

    /// Add a u8 to A
    ///
    /// 2 cycles
    fn adc(&mut self, n8: u8) {
        let (temp, overflow) = n8
            .overflowing_add(self.carry_bit() as u8);
        let (res, overflow2) = self.a_reg()
            .overflowing_add(temp);
        let overflow = overflow || overflow2;

        let half_overflow = (
            (self.a_reg() & 0xF) +
                (n8 & 0xF) +
                self.carry_bit() as u8
        ) > 0xF; // Does adding the lower half of the numbers (plus carry) overflow?

        *self.f_reg_mut() = 0; // Reset flag register

        self.set_carry_bit(overflow); // Did the calculation overflow
        self.set_half_carry_bit(half_overflow); // See half_overflow
        self.set_zero_bit(res == 0); // Is the result zero

        *self.a_reg_mut() = res;
    }

    /// Add the value in the register to A
    ///
    /// 1 cycle
    fn add_reg(&mut self, reg: Register8) {
        self.add(self.reg(reg));
    }

    /// Add the value HL points to to A
    ///
    /// 2 cycles
    fn add_hl(&mut self) {
        self.add(self.mmu.read_8(self.read_hl()))
    }

    /// Add a u8 to A
    ///
    /// 2 cycles
    fn add(&mut self, n8: u8) {
        let (res, overflow) = self.a_reg()
            .overflowing_add(n8);

        let half_overflow = (
            (self.a_reg() & 0xF) +
            (n8 & 0xF)
        ) > 0xF; // Does adding the lower half of the numbers (plus carry) overflow?

        *self.f_reg_mut() = 0; // Reset flag register

        self.set_carry_bit(overflow); // Did the calculation overflow
        self.set_half_carry_bit(half_overflow); // See half_overflow
        self.set_zero_bit(res == 0); // Is the result zero

        *self.a_reg_mut() = res;
    }

    /// Add a 16 bit register to HL
    ///
    /// 2 cycles
    fn add_r16_to_hl(&mut self, reg: Register16) {
        self.add_n16_to_hl(self.reg_16(reg));
    }

    /// Add the value in SP to HL
    ///
    /// 2 cycles
    fn add_sp_to_hl(&mut self) {
        self.add_n16_to_hl(self.sp);
    }

    /// Add a u16 to HL
    ///
    /// Actually not supported by the GB Classic
    fn add_n16_to_hl(&mut self, n16: u16) {
        let hl = self.read_hl();
        let (res, overflow) = hl.overflowing_add(n16);

        let half_overflow = (
            (hl & 0xFFF) +
            (n16 & 0xFFF)
        ) > 0xF; // Does adding the lower half of the numbers (plus carry) overflow?

        *self.f_reg_mut() = 0; // Reset flag register

        self.set_carry_bit(overflow); // Did the calculation overflow
        self.set_half_carry_bit(half_overflow); // See half_overflow
        // zero bit is not set

        self.write_hl(res);
    }

    /// Add the signed value e8 to SP
    ///
    /// 4 cycles
    fn add_e8_to_sp(&mut self, e8: i8) {
        // https://github.com/aidan-clyens/GBExperience/blob/master/src/cpu/cpu_alu.cpp#L375-L387
        let res = (self.sp as i32 + e8) as u16; // TODO check out

        // Reset flag register
        *self.f_reg_mut() = 0;
        // TODO WTF???
        self.set_carry_bit((self.sp ^ e8 ^ (result & 0xFFFF)) & 0x100 == 0x100);
        // TODO WTF???
        self.set_half_carry_bit((self.sp ^ e8 ^ (result & 0xFFFF)) & 0x10 == 0x10)

        self.sp = res;
    }
    }
}
