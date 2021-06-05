
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

enum Condition {
    ZSet,
    ZNotSet,
    CSet,
    CNotSet
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
        let res = (self.sp as i32 + e8 as i32) as u16; // TODO check out

        // Normal casting doesn't do the correct thing
        let e8_byte = e8.to_le_bytes()[0];

        // Reset flag register
        *self.f_reg_mut() = 0;
        // TODO WTF???
        self.set_carry_bit((self.sp ^ (e8_byte as u16) ^ (res & 0xFFFF)) & 0x100 == 0x100);
        // TODO WTF???
        self.set_half_carry_bit((self.sp ^ (e8_byte as u16) ^ (res & 0xFFFF)) & 0x10 == 0x10);

        self.sp = res;
    }

    /// Calculate the bitwise and between the register and A and store it in A
    ///
    /// 1 cycle
    fn and_reg(&mut self, reg: Register8) {
        self.and(self.reg(reg));
    }

    /// Calculate the bitwise and between the byte pointed to by HL and A and store it in A
    ///
    /// 2 Cycles
    fn and_hl(&mut self) {
        self.and(self.mmu.read_8(self.read_hl()))
    }

    /// Calculate the bitwise and between the number and A and store it in A
    ///
    /// Either 2
    fn and(&mut self, n8: u8) {
        let res = self.a_reg() & n8;

        // Reset flag register
        *self.f_reg_mut() = 0;
        self.set_zero_bit(res == 0); // Set if result is zero
        self.set_half_carry_bit(true); // By definition

        *self.a_reg_mut() = res;
    }

    /// Check if the specified bit is set in the register.
    /// The zero flag is set, if the bit was not set.
    ///
    /// 2 cycles
    fn bit_reg(&mut self, reg: Register8, bit: u8) {
        self.bit(self.reg(reg), bit);
    }

    /// Check if the specified bit is set in the byte that HL points to.
    /// The zero flag is set, if the bit was not set.
    ///
    /// 3 cycles
    fn bit_hl(&mut self, bit: u8) {
        self.bit(self.mmu.read_8(self.read_hl()), bit);
    }

    /// Test if the specified bit of the byte is set and set the zero flag IF NOT set
    ///
    /// This technically doesn't exist for arbitrary bytes in the GB Classic
    fn bit(&mut self, to_test: u8, bit: u8) {
        assert!(bit <= 7); // Is an assert necessary?
        // This somehow doesn't set (or reset) the carry flag
        self.set_zero_bit(to_test & (1 << bit) == 0);
        self.set_negative_bit(false); // By definition
        self.set_half_carry_bit(true); // By definition
    }

    fn call(&mut self, n16: u16) {
        // Call address n16. This pushes the address of the instruction after
        // the CALL on the stack, such that RET can pop it later; then,
        // it executes an implicit JP n16.
        todo!();
    }

    fn call_cc(&mut self, n16: u16) {
        // Call address n16, if condition cc is met (see call)
        todo!()
    }

    /// Complement (invert) the carry flag.
    ///
    /// 1 cycle
    fn ccf(&mut self) {
        self.set_negative_bit(false); // By definition
        self.set_half_carry_bit(false); // By definition
        self.set_carry_bit(!self.carry_bit());
    }

    /// Subtract the value in reg from A, but only set the flags and don't store the result
    ///
    /// 1 cycle
    fn cp_reg(&mut self, reg: Register8) {
        self.cp(self.reg(reg));
    }

    /// Subtract the value of the byte pointed to by HL from A,
    /// but only set the flags and don't store the result
    ///
    /// 2 cycles
    fn cp_hl(&mut self) {
        self.cp(self.mmu.read_8(self.read_hl()))
    }

    /// Subtract n8 from A, but only set the flags and don't store the result
    ///
    /// 2 cycles
    fn cp(&mut self, n8: u8) {
        let a = self.a_reg();
        self.set_zero_bit(a == n8); // Result is only zero, if A == n8
        self.set_negative_bit(true);
        // Result of lower nibble would have to borrow
        self.set_half_carry_bit((n8 & 0xF) > (a & 0xF));
        self.set_carry_bit(n8 > a); // Result would have to borrow
    }

    /// Complement the Accumulator/A register (A = ~A)
    ///
    /// 1 cycle
    fn cpl(&mut self) {
        *self.a_reg_mut() = !self.a_reg();
        self.set_negative_bit(true); // By definition
        self.set_half_carry_bit(true); // By definition
    }

    /// Decimal Adjust Accumulator (A register) to get correct BCD representation
    /// (see https://ehaskins.com/2018-01-30%20Z80%20DAA/)
    ///
    /// 1 cycle
    fn daa(&mut self) {
        // TODO understand
        // No idea how this works xD see link above
        let mut correction: u8 = 0;
        let mut set_carry = false;

        let mut val = self.a_reg() as i16;

        if self.half_carry_bit() || (!self.negative_bit() && (val & 0xF) > 9) {
            correction |= 0x6;
        }

        if self.carry_bit() || (!self.negative_bit() && (val > 0x99)) {
            correction |= 0x60;
            set_carry = true;
        }

        let correction = correction as i16;

        val += if self.negative_bit() { -correction } else { correction };

        let val = (val & 0xFF) as u8; // This should be the same as %= 0xFF

        self.set_half_carry_bit(false);
        self.set_carry_bit(set_carry);
        self.set_zero_bit(val == 0);

        *self.a_reg_mut() = val;
    }

    /// Decrement the value of the specified register by 1
    ///
    /// 1 cycle
    fn dec_reg8(&mut self, reg: Register8) {
        // TODO I have no idea if this is correct!
        // Set the half carry bit if borrowing from bit 4
        // This is the case if the lower nibble is zero
        self.set_half_carry_bit(dbg!(self.reg(reg) & 0xF) == 0);
        *self.reg_mut(reg) = self.reg(reg) - 1;
        self.set_zero_bit(self.reg(reg) == 0); // Set flag if result is zero
        self.set_negative_bit(true); // By specification
    }

    /// Decrement the value of the byte pointed to by HL by one
    ///
    /// 3 cycles
    fn dec_hl(&mut self) {
        let hl = self.read_hl();
        let mut val = self.mmu.read_8(hl);

        // TODO no idea if this is correct
        // Set the half carry bit if borrowing from bit 4
        // This is the case if the lower nibble is zero
        self.set_half_carry_bit(dbg!(val & 0xF) == 0);
        val -= 1;

        self.set_zero_bit(val == 0);
        self.set_negative_bit(true); // By definition
        self.mmu.write_8(hl, val);
    }

    /// Decrement the value of the specified 16 bit register
    ///
    /// 2 cycles
    fn dec_reg16(&mut self, reg: Register16) {
        self.write_reg_16(self.reg_16(reg) - 1, reg);
    }

    /// Decrement SP by 1
    ///
    /// 2 cycles
    fn dec_sp(&mut self) {
        self.sp -= 1;
    }

    /// Disable interrupts by clearing the IME flag
    ///
    /// 1 cycle
    fn di(&mut self) {
        todo!("DI: No idea what flag to clear here");
    }

    /// Enable interrupts by setting the IME flag.
    /// Normally only set AFTER the instruction following this one
    ///
    /// 1 cycle
    fn ei(&mut self) {
        todo!("EI: No idea what flag to set here");
    }

    /// Halt the CPU and set it in low power mode until an interrupt occurs.
    /// This one is not implemented yet
    ///
    /// - cycles
    fn halt(&mut self) {
        todo!("HALT: This one has complicated behavior")
    }

    /// Increment the specified register by 1.
    ///
    /// 1 cycle
    fn inc_r8(&mut self, reg: Register8) {
        // TODO no idea if that is correct
        self.set_half_carry_bit(dbg!(self.reg(reg) & 0xF) == 0xF);
        self.set_negative_bit(false); // By definition

        *self.reg_mut(reg) = self.reg(reg) + 1;

        self.set_zero_bit(self.reg(reg) == 0);
    }

    /// Increment the byte pointed to by HL by 1
    ///
    /// 3 cycles
    fn inc_hl(&mut self) {
        let hl = self.read_hl();
        let mut val = self.mmu.read_8(hl);
        // TODO no idea if this is correct
        self.set_half_carry_bit(dbg!(val & 0xF) == 0xF);
        self.set_negative_bit(false); // By definition

        // TODO might need overflowing add
        val += 1;

        self.set_zero_bit(val == 0);
        // TODO so far unused MemResult
        self.mmu.write_8(hl, val);
    }

    /// Increment the value of the specified 16 bit register by 1
    ///
    /// 2 cycles
    fn inc_r16(&mut self, reg: Register16) {
        self.write_reg_16(self.reg_16(reg) + 1, reg);
    }

    /// Increment SP by 1
    ///
    /// 2 cycles
    fn inc_sp(&mut self) {
        self.sp += 1;
    }

    /// Jump to address n16 by setting PC to n16
    ///
    /// 4 cycles
    fn jp(&mut self, n16: u16) {
        self.pc = n16;
    }

    /// Jump to the address n16 by setting PC to n16, if the condition cc is met
    ///
    /// 4 taken cycles, 3 untaken cycles
    /// -> I'm pretty sure this means:
    ///    - 4 cycles if condition is met
    ///    - 3 cycles if condition is not met
    fn jp_cc(&mut self, cc: Condition, n16: u16) {
        if self.check_condition(cc) {
            self.pc = n16;
        }
    }

    /// Jump to the value of the HL register, effectively setting PC to HL
    ///
    /// 1 cycle
    fn jp_hl(&mut self) {
        self.pc = self.read_hl();
    }

    /// Jump relative by adding e8 to the address of the instruction FOLLOWING JR.
    /// `e8 == 0` would be equivalent to no jump
    ///
    /// 3 cycles
    fn jr(&mut self, e8: i8) {
        if e8 < 0 {
            let jump = (-e8) as u16;
            self.pc.overflowing_sub(jump);
        } else {
            self.pc.overflowing_add(e8 as u16);
        }
    }

    /// Jump relative by adding e8 to the address of the instruction FOLLOWING JR,
    /// if the condition is met.
    /// `e8 == 0` would be equivalent to no jump
    ///
    /// 3 cycles if condition is met
    /// 2 cycles if condition is not met
    fn jr_cc(&mut self, cc: Condition, e8: i8) {
        if self.check_condition(cc) {
            self.jr(e8);
        }
    }

    /// Load (copy) the value from the register on the right to the register on the left.
    ///
    /// 1 cycle
    fn ld_r8_to_r8(&mut self, to: Register8, from: Register8) {
        self.ld_const8_to_r8(to, self.reg(from));
    }

    /// Load the constant into the specified register
    ///
    /// 2 cycles
    fn ld_const8_to_r8(&mut self, to: Register8, n8: u8) {
        *self.reg_mut(to) = n8;
    }

    /// Load n16 value into specified 16 bit register
    ///
    /// 3 cycles
    fn ld_const16_to_r16(&mut self, to: Register16, n16: u16) {
        self.write_reg_16(n16, to);
    }

    /// Store value from specified register into byte pointed to by HL
    ///
    /// 2 cycles
    fn ld_r8_to_hl(&mut self, from: Register8) {
        self.ld_const8_to_hl(self.reg(from));
    }

    /// Store the specified byte into the byte pointed to by HL
    ///
    /// 3 cycles
    fn ld_const8_to_hl(&mut self, n8: u8) {
        self.mmu.write_8(self.read_hl(), n8);
    }

    /// Store the value pointed to by HL into the specified register
    ///
    /// 2 cycles
    fn ld_hl_to_r8(&mut self, to: Register8) {
        *self.reg_mut(to) = self.mmu.read_8(self.read_hl());
    }

    /// Store the value in the A register into the address pointed to by the specified register
    ///
    /// 2 cycles
    fn ld_a_to_r16addr(&mut self, reg: Register16) {
        self.ld_a_to_const16addr(self.reg_16(reg));
    }

    /// Store the value in the A register into the byte at the specified address
    ///
    /// 4 cycles
    fn ld_a_to_const16addr(&mut self, n16: u16) {
        self.mmu.write_8(n16, self.a_reg());
    }

    /// Store the value in the A register into the byte at the specified address, provided
    /// the address is between 0xFF00 and 0xFFFF (I'm pretty sure both inclusive)
    ///
    /// 3 cycles
    fn ldh_a_to_const16addr(&mut self, n16: u16) {
        // I'm pretty sure this is meant as a guarantee and not as a noop if
        // the condition is not met
        assert!(n16 >= 0xFF00u16 && n16 <= 0xFFFFu16);
        self.ld_a_to_const16addr(n16);
    }

    /// Store the value in register A into the byte at address 0xFF00 + C (register)
    ///
    /// 2 cycles
    fn ldh_a_to_ff00_plus_C(&mut self) {
        self.ldh_a_to_const16addr(0xFF00 + self.c_reg() as u16);
    }

    /// Load value into register A from byte pointed to by the specified register
    ///
    /// 2 cycles
    fn ld_r16addr_to_a(&mut self, reg: Register16) {
        self.ld_const16addr_to_a(self.reg_16(reg));
    }

    /// Load value into register A from byte pointed to by the specified address
    ///
    /// 4 cycles
    fn ld_const16addr_to_a(&mut self, n16: u16) {
        *self.a_reg_mut() = self.mmu.read_8(n16);
    }

    /// Load value into register A from byte pointed to by the specified address, provided, the
    /// address is between 0xFF00 and 0xFFFF (both inclusive)
    ///
    /// 3 cycles
    fn ldh_const16addr_to_a(&mut self, n16: u16) {
        // I'm pretty sure this is meant as a guarantee and not as a noop if
        // the condition is not met
        assert!(n16 >= 0xFF00u16 && n16 <= 0xFFFFu16);
        self.ld_const16addr_to_a(n16);
    }

    /// Load value into register A from the byte at address 0xFF00 + C (register)
    ///
    /// 2 cycles
    fn ldh_ff00_plus_C_to_a(&mut self) {
        self.ldh_const16addr_to_a(0xFF00 + self.c_reg() as u16);
    }

    /// Load value from register A into byte pointed to by HL and increment HL
    ///
    /// 2 cycles
    fn ld_a_to_hl_and_inc(&mut self) {
        self.ld_r8_to_hl(Register8::A);
        self.inc_hl();
    }

    /// Load value from register A into byte pointed to by HL and decrement HL
    ///
    /// 2 cycles
    fn ld_a_to_hl_and_dec(&mut self) {
        self.ld_r8_to_hl(Register8::A);
        self.dec_hl();
    }

    /// Load value into register A from byte pointed to by HL and increment HL
    ///
    /// 2 cycles
    fn ld_hl_to_a_and_inc(&mut self) {
        self.ld_hl_to_r8(Register8::A);
        self.inc_hl();
    }

    /// Load value into register A from byte pointed to by HL and decrement HL
    ///
    /// 2 cycles
    fn ld_hl_to_a_and_dec(&mut self) {
        self.ld_hl_to_r8(Register8::A);
        self.dec_hl();
    }

    /// Load specified value into SP
    ///
    /// 3 cycles
    fn ld_const16_to_sp(&mut self, n16: u16) {
        self.sp = n16;
    }

    /// Store SP & $FF at address n16 and SP >> 8 at address n16 + 1.
    /// This is a weird one. xD
    ///
    /// 5 cycles
    fn ld_sp_to_const16addr(&mut self, n16: u16) {
        self.mmu.write_8(n16, (self.sp & 0xFF) as u8);
        self.mmu.write_8(n16 + 1, (self.sp >> 8) as u8);
    }

    /// Add the signed value e8 to SP and store the result in HL.
    ///
    /// 3 cycles
    fn ld_sp_plus_e8_to_hl(&mut self, e8: i8) {
        let res = if e8 < 0 {
            let add = e8 as u8;
            let (res, overflow) = self.sp.overflowing_add(add as u16);
            // TODO check if flags are correctly set
            self.set_carry_bit((
                (self.sp & 0xFF) +
                (add as u16)
            ) > 0xFF);
            self.set_half_carry_bit((
                (self.sp & 0xF) +
                (add as u16 & 0xF)
            ) > 0xF);
            res
        } else {
            // I think if the jump is subtracting, the overflow flags are set to zero
            self.set_half_carry_bit(false);
            self.set_carry_bit(false);
            let sub = (-e8) as u16;
            let (res, _) = self.sp.overflowing_sub(sub);
            res
        };
        self.set_zero_bit(false); // By definition
        self.set_negative_bit(false); // By definition

        self.write_hl(res);
    }

    /// Load register HL into SP
    ///
    /// 2 cycles
    fn ld_hl_to_sp(&mut self) {
        self.sp = self.read_hl();
    }

    /// For completeness
    ///
    /// 1 cycle
    fn nop() { }

    /// Calculate the bitwise or between register A and the specified register and
    /// store the result in register A.
    ///
    /// 1 cycle
    fn or_reg(&mut self, reg: Register8) {
        self.or(self.reg(reg));
    }

    /// Calculate the bitwise or between register A and the byte pointed to by HL
    /// and store the result in register A.
    ///
    /// 2 cycles
    fn or_hl(&mut self) {
        self.or(self.mmu.read_8(self.read_hl()));
    }

    /// Calculate the bitwise or between register A and the specified byte and
    /// store the result in register A.
    ///
    /// 2 cycles
    fn or(&mut self, n8: u8) {
        let res = self.a_reg() | n8;
        self.set_zero_bit(res == 0);
        self.set_half_carry_bit(false);
        self.set_carry_bit(false);
        self.set_negative_bit(false);
        *self.a_reg_mut() = res;
    }
}
