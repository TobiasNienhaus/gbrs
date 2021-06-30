use super::*;

impl Cpu {
    /// Add the value in <reg> to A plus the carry flag
    ///
    /// Takes 1 cycle
    pub(super) fn adc_r8(&mut self, reg: Register8) -> u32 {
        self.adc(self.reg(reg));
        1
    }

    /// Add the value that HL points to to A plus the carry flag
    ///
    /// 2 cycles
    pub(super) fn adc_hl(&mut self) -> u32 {
        self.adc(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Add a u8 to A
    ///
    /// 2 cycles
    pub(super) fn adc(&mut self, n8: u8) -> u32 {
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
        2
    }

    /// Add the value in the register to A
    ///
    /// 1 cycle
    pub(super) fn add_r8(&mut self, reg: Register8) -> u32 {
        self.add(self.reg(reg));
        1
    }

    /// Add the value HL points to to A
    ///
    /// 2 cycles
    pub(super) fn add_hl(&mut self) -> u32 {
        self.add(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Add a u8 to A
    ///
    /// 2 cycles
    pub(super) fn add(&mut self, n8: u8) -> u32 {
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
        2
    }

    /// Add a 16 bit register to HL
    ///
    /// 2 cycles
    pub(super) fn add_r16_to_hl(&mut self, reg: Register16) -> u32 {
        self.add_n16_to_hl(self.reg16(reg));
        2
    }

    /// Add the value in SP to HL
    ///
    /// 2 cycles
    pub(super) fn add_sp_to_hl(&mut self) -> u32 {
        self.add_n16_to_hl(self.sp);
        2
    }

    /// Add a u16 to HL
    ///
    /// Actually not supported by the GB Classic
    fn add_n16_to_hl(&mut self, n16: u16) {
        let hl = self.reg16(Register16::HL);
        let (res, overflow) = hl.overflowing_add(n16);

        // Does adding the lower half of the numbers (plus carry) overflow?
        let half_overflow = ((hl & 0xFFF) + (n16 & 0xFFF)) > 0xFFF;

        self.set_negative_bit(false); // By definition

        self.set_carry_bit(overflow); // Did the calculation overflow
        self.set_half_carry_bit(half_overflow); // See half_overflow
        // zero bit is not set

        self.write_reg16(Register16::HL, res);
    }

    /// Add the signed value e8 to SP
    ///
    /// 4 cycles
    pub(super) fn add_e8_to_sp(&mut self, e8: i8) -> u32 {
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
        4
    }

    /// Calculate the bitwise and between the register and A and store it in A
    ///
    /// 1 cycle
    pub(super) fn and_r8(&mut self, reg: Register8) -> u32 {
        self.and(self.reg(reg));
        1
    }

    /// Calculate the bitwise and between the byte pointed to by HL and A and store it in A
    ///
    /// 2 Cycles
    pub(super) fn and_hl(&mut self) -> u32 {
        self.and(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Calculate the bitwise and between the number and A and store it in A
    ///
    /// 2 cycles
    pub(super) fn and(&mut self, n8: u8) -> u32 {
        let res = self.a_reg() & n8;

        // Reset flag register
        *self.f_reg_mut() = 0;
        self.set_zero_bit(res == 0); // Set if result is zero
        self.set_half_carry_bit(true); // By definition

        *self.a_reg_mut() = res;
        2
    }

    /// Check if the specified bit is set in the register.
    /// The zero flag is set, if the bit was not set.
    ///
    /// 2 cycles
    pub(super) fn bit_r8(&mut self, reg: Register8, bit: u8) -> u32 {
        self.bit(self.reg(reg), bit);
        2
    }

    /// Check if the specified bit is set in the byte that HL points to.
    /// The zero flag is set, if the bit was not set.
    ///
    /// 3 cycles
    pub(super) fn bit_hl(&mut self, bit: u8) -> u32 {
        self.bit(self.mmu.read_8(self.reg16(Register16::HL)), bit);
        3
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

    /// Call address n16. This pushes the address of the instruction after the CALL on the stack,
    /// such that RET can pop it later; then, it executes an implicit JP n16.
    ///
    /// 6 cycles
    pub(super) fn call(&mut self, n16: u16) -> u32 {
        // Call address n16. This pushes the address of the instruction after
        // the CALL on the stack, such that RET can pop it later; then,
        // it executes an implicit JP n16.
        // TODO check if correct
        self.push_n16(self.pc);
        self.pc = n16;
        6
    }

    /// Call address n16 if condition cc is met. (See call)
    ///
    /// 6 cycles taken / 3 cycles untaken
    pub(super) fn call_cc(&mut self, cc: Condition, n16: u16) -> u32 {
        // Call address n16, if condition cc is met (see call)
        if self.check_condition(cc) {
            self.call(n16);
            6
        } else {
            3
        }
    }

    /// Complement (invert) the carry flag.
    ///
    /// 1 cycle
    pub(super) fn ccf(&mut self) -> u32 {
        self.set_negative_bit(false); // By definition
        self.set_half_carry_bit(false); // By definition
        self.set_carry_bit(!self.carry_bit());
        1
    }

    /// Subtract the value in reg from A, but only set the flags and don't store the result
    ///
    /// 1 cycle
    pub(super) fn cp_r8(&mut self, reg: Register8) -> u32 {
        self.cp(self.reg(reg));
        1
    }

    /// Subtract the value of the byte pointed to by HL from A,
    /// but only set the flags and don't store the result
    ///
    /// 2 cycles
    pub(super) fn cp_hl(&mut self) -> u32 {
        self.cp(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Subtract n8 from A, but only set the flags and don't store the result
    ///
    /// 2 cycles
    pub(super) fn cp(&mut self, n8: u8) -> u32 {
        let a = self.a_reg();
        self.set_zero_bit(a == n8); // Result is only zero, if A == n8
        self.set_negative_bit(true);
        // Result of lower nibble would have to borrow
        self.set_half_carry_bit((n8 & 0xF) > (a & 0xF));
        self.set_carry_bit(n8 > a); // Result would have to borrow
        2
    }

    /// Complement the Accumulator/A register (A = ~A)
    ///
    /// 1 cycle
    pub(super) fn cpl(&mut self) -> u32 {
        *self.a_reg_mut() = !self.a_reg();
        self.set_negative_bit(true); // By definition
        self.set_half_carry_bit(true); // By definition
        1
    }

    /// Decimal Adjust Accumulator (A register) to get correct BCD representation
    /// (see https://ehaskins.com/2018-01-30%20Z80%20DAA/)
    ///
    /// 1 cycle
    pub(super) fn daa(&mut self) -> u32 {
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

        1
    }

    /// Decrement the value of the specified register by 1
    ///
    /// 1 cycle
    pub(super) fn dec_r8(&mut self, reg: Register8) -> u32 {
        // Set the half carry bit if borrowing from bit 4
        // This is the case if the lower nibble is zero
        self.set_half_carry_bit((self.reg(reg) & 0xF) == 0);
        *self.reg_mut(reg) = self.reg(reg).overflowing_sub(1).0;
        self.set_zero_bit(self.reg(reg) == 0); // Set flag if result is zero
        self.set_negative_bit(true); // By definition
        1
    }

    /// Decrement the value of the byte pointed to by HL by one
    ///
    /// 3 cycles
    pub(super) fn dec_hl(&mut self) -> u32 {
        let hl = self.reg16(Register16::HL);
        let mut val = self.mmu.read_8(hl);

        // Set the half carry bit if borrowing from bit 4
        // This is the case if the lower nibble is zero
        self.set_half_carry_bit((val & 0xF) == 0);
        val = val.overflowing_sub(1).0;

        self.set_zero_bit(val == 0);
        self.set_negative_bit(true); // By definition
        self.mmu.write_8(hl, val);
        3
    }

    /// Decrement the value of the specified 16 bit register
    ///
    /// 2 cycles
    pub(super) fn dec_r16(&mut self, reg: Register16) -> u32 {
        self.write_reg16(reg, self.reg16(reg) - 1);
        2
    }

    /// Decrement SP by 1
    ///
    /// 2 cycles
    pub(super) fn dec_sp(&mut self) -> u32 {
        self.sp -= 1;
        2
    }

    /// Disable interrupts by clearing the IME flag
    ///
    /// 1 cycle
    pub(super) fn di(&mut self) -> u32 {
        // TODO shouldn't this set a bit at some address in memory?
        self.interrupts_enabled = false;
        1
    }

    /// Enable interrupts by setting the IME flag.
    /// Normally only set AFTER the instruction following this one
    ///
    /// 1 cycle
    pub(super) fn ei(&mut self) -> u32 {
        // TODO shouldn't this set a bit at some address in memory?
        self.interrupts_enabled = true;
        1
    }

    /// Halt the CPU and set it in low power mode until an interrupt occurs.
    ///
    /// - cycles
    pub(super) fn halt(&mut self) -> u32 {
        // TODO check if this is the correct behavior
        self.halted = true;
        1 // Is this correct?
    }

    /// Increment the specified register by 1.
    ///
    /// 1 cycle
    pub(super) fn inc_r8(&mut self, reg: Register8) -> u32 {
        // Set half carry, if the instruction would overflow after the 3rd bit
        // This means the first nibble is completely set, so 0xF
        self.set_half_carry_bit((self.reg(reg) & 0xF) == 0xF);
        self.set_negative_bit(false); // By definition

        *self.reg_mut(reg) = self.reg(reg) + 1;

        self.set_zero_bit(self.reg(reg) == 0);
        1
    }

    /// Increment the byte pointed to by HL by 1
    ///
    /// 3 cycles
    pub(super) fn inc_hl(&mut self) -> u32 {
        let hl = self.reg16(Register16::HL);
        let mut val = self.mmu.read_8(hl);
        // Set half carry, if the instruction would overflow after the 3rd bit
        // This means the first nibble is completely set, so 0xF
        self.set_half_carry_bit((val & 0xF) == 0xF);
        self.set_negative_bit(false); // By definition

        val = val.overflowing_add(1).0;

        self.set_zero_bit(val == 0);
        // TODO so far unused MemResult
        self.mmu.write_8(hl, val);
        3
    }

    /// Increment the value of the specified 16 bit register by 1
    ///
    /// 2 cycles
    pub(super) fn inc_r16(&mut self, reg: Register16) -> u32 {
        self.write_reg16(reg, self.reg16(reg).overflowing_add(1).0);
        2
    }

    /// Increment SP by 1
    ///
    /// 2 cycles
    pub(super) fn inc_sp(&mut self) -> u32 {
        self.sp += 1;
        2
    }

    /// Jump to address n16 by setting PC to n16
    ///
    /// 4 cycles
    pub(super) fn jp(&mut self, n16: u16) -> u32 {
        self.pc = n16;
        4
    }

    /// Jump to the address n16 by setting PC to n16, if the condition cc is met
    ///
    /// 4 taken cycles, 3 untaken cycles
    /// -> I'm pretty sure this means:
    ///    - 4 cycles if condition is met
    ///    - 3 cycles if condition is not met
    pub(super) fn jp_cc(&mut self, cc: Condition, n16: u16) -> u32 {
        if self.check_condition(cc) {
            self.pc = n16;
            4
        } else {
            3
        }
    }

    /// Jump to the value of the HL register, effectively setting PC to HL
    ///
    /// 1 cycle
    pub(super) fn jp_hl(&mut self) -> u32 {
        self.pc = self.reg16(Register16::HL);
        1
    }

    /// Jump relative by adding e8 to the address of the instruction FOLLOWING JR.
    /// `e8 == 0` would be equivalent to no jump
    ///
    /// 3 cycles
    pub(super) fn jr(&mut self, e8: i8) -> u32 {
        if e8 < 0 {
            let jump = (-e8) as u16;
            self.pc = self.pc.overflowing_sub(jump).0;
        } else {
            self.pc = self.pc.overflowing_add(e8 as u16).0;
        }
        3
    }

    /// Jump relative by adding e8 to the address of the instruction FOLLOWING JR,
    /// if the condition is met.
    /// `e8 == 0` would be equivalent to no jump
    ///
    /// 3 cycles if condition is met
    /// 2 cycles if condition is not met
    pub(super) fn jr_cc(&mut self, cc: Condition, e8: i8) -> u32 {
        if self.check_condition(cc) {
            self.jr(e8);
            3
        } else {
            2
        }
    }

    /// Load (copy) the value from the register on the right to the register on the left.
    ///
    /// 1 cycle
    pub(super) fn ld_r8_to_r8(&mut self, to: Register8, from: Register8) -> u32 {
        self.ld_const8_to_r8(to, self.reg(from));
        1
    }

    /// Load the constant into the specified register
    ///
    /// 2 cycles
    pub(super) fn ld_const8_to_r8(&mut self, to: Register8, n8: u8) -> u32 {
        *self.reg_mut(to) = n8;
        2
    }

    /// Load n16 value into specified 16 bit register
    ///
    /// 3 cycles
    pub(super) fn ld_const16_to_r16(&mut self, to: Register16, n16: u16) -> u32 {
        self.write_reg16(to, n16);
        3
    }

    /// Store value from specified register into byte pointed to by HL
    ///
    /// 2 cycles
    pub(super) fn ld_r8_to_hl(&mut self, from: Register8) -> u32 {
        self.ld_const8_to_hl(self.reg(from));
        2
    }

    /// Store the specified byte into the byte pointed to by HL
    ///
    /// 3 cycles
    pub(super) fn ld_const8_to_hl(&mut self, n8: u8) -> u32 {
        self.ld_const8_to_const16addr(n8, self.reg16(Register16::HL));
        3
    }

    /// Basically LD [n16],[n8]
    ///
    /// Does not exist on GB Classic
    fn ld_const8_to_const16addr(&mut self, n8: u8, n16: u16) {
        self.mmu.write_8(n16, n8);
    }

    /// Store the value pointed to by HL into the specified register
    ///
    /// 2 cycles
    pub(super) fn ld_hl_to_r8(&mut self, to: Register8) -> u32 {
        *self.reg_mut(to) = self.mmu.read_8(self.reg16(Register16::HL));
        2
    }

    /// Store the value in the A register into the address pointed to by the specified register
    ///
    /// 2 cycles
    pub(super) fn ld_a_to_r16addr(&mut self, reg: Register16) -> u32 {
        self.ld_a_to_const16addr(self.reg16(reg));
        2
    }

    /// Store the value in the A register into the byte at the specified address
    ///
    /// 4 cycles
    pub(super) fn ld_a_to_const16addr(&mut self, n16: u16) -> u32 {
        self.mmu.write_8(n16, self.a_reg());
        4
    }

    /// Store the value in the A register into the byte at the specified address, provided
    /// the address is between 0xFF00 and 0xFFFF (I'm pretty sure both inclusive)
    ///
    /// 3 cycles
    pub(super) fn ldh_a_to_const16addr(&mut self, n16: u16) -> u32 {
        // I'm pretty sure this is meant as a guarantee and not as a noop if
        // the condition is not met
        assert!(n16 >= 0xFF00u16 && n16 <= 0xFFFFu16);
        self.ld_a_to_const16addr(n16);
        3
    }

    /// Store the value in register A into the byte at address 0xFF00 + C (register)
    ///
    /// 2 cycles
    pub(super) fn ldh_a_to_ff00_plus_c(&mut self) -> u32 {
        self.ldh_a_to_const16addr(0xFF00 + self.c_reg() as u16);
        2
    }

    /// Load value into register A from byte pointed to by the specified register
    ///
    /// 2 cycles
    pub(super) fn ld_r16addr_to_a(&mut self, reg: Register16) -> u32 {
        self.ld_const16addr_to_a(self.reg16(reg));
        2
    }

    /// Load value into register A from byte pointed to by the specified address
    ///
    /// 4 cycles
    pub(super) fn ld_const16addr_to_a(&mut self, n16: u16) -> u32 {
        self.ld_const16addr_to_r8(n16, Register8::A);
        4
    }

    /// Load value into specified register from byte pointed to by the specified address
    ///
    /// Does not exist in GB classic
    fn ld_const16addr_to_r8(&mut self, n16: u16, to: Register8) {
        *self.reg_mut(to) = self.mmu.read_8(n16);
    }

    /// Load value from specified register into byte pointed to by the specified address
    ///
    /// Does not exist on the GB classic
    fn ld_r8_to_const16addr(&mut self, from: Register8, n16: u16) {
        self.mmu.write_8(n16, self.reg(from));
    }

    /// Load value into register A from byte pointed to by the specified address, provided, the
    /// address is between 0xFF00 and 0xFFFF (both inclusive)
    ///
    /// 3 cycles
    pub(super) fn ldh_const16addr_to_a(&mut self, n16: u16) -> u32 {
        // I'm pretty sure this is meant as a guarantee and not as a noop if
        // the condition is not met
        assert!(n16 >= 0xFF00u16 && n16 <= 0xFFFFu16);
        self.ld_const16addr_to_a(n16);
        3
    }

    /// Load value into register A from the byte at address 0xFF00 + C (register)
    ///
    /// 2 cycles
    pub(super) fn ldh_ff00_plus_c_to_a(&mut self) -> u32 {
        self.ldh_const16addr_to_a(0xFF00 + self.c_reg() as u16);
        2
    }

    /// Load value from register A into byte pointed to by HL and increment HL
    ///
    /// 2 cycles
    pub(super) fn ld_a_to_hl_and_inc(&mut self) -> u32 {
        self.ld_r8_to_hl(Register8::A);
        self.inc_hl();
        2
    }

    /// Load value from register A into byte pointed to by HL and decrement HL
    ///
    /// 2 cycles
    pub(super) fn ld_a_to_hl_and_dec(&mut self) -> u32 {
        self.dec_hl();
        self.ld_r8_to_hl(Register8::A);
        2
    }

    /// Load value into register A from byte pointed to by HL and increment HL
    ///
    /// 2 cycles
    pub(super) fn ld_hl_to_a_and_inc(&mut self) -> u32 {
        self.ld_hl_to_r8(Register8::A);
        self.inc_hl();
        2
    }

    /// Load value into register A from byte pointed to by HL and decrement HL
    ///
    /// 2 cycles
    pub(super) fn ld_hl_to_a_and_dec(&mut self) -> u32 {
        self.dec_hl();
        self.ld_hl_to_r8(Register8::A);
        2
    }

    /// Load specified value into SP
    ///
    /// 3 cycles
    pub(super) fn ld_const16_to_sp(&mut self, n16: u16) -> u32 {
        self.sp = n16;
        3
    }

    /// Store SP & $FF at address n16 and SP >> 8 at address n16 + 1.
    /// This is a weird one. xD
    ///
    /// 5 cycles
    pub(super) fn ld_sp_to_const16addr(&mut self, n16: u16) -> u32 {
        self.mmu.write_8(n16, (self.sp & 0xFF) as u8);
        self.mmu.write_8(n16 + 1, (self.sp >> 8) as u8);
        5
    }

    /// Add the signed value e8 to SP and store the result in HL.
    ///
    /// 3 cycles
    pub(super) fn ld_sp_plus_e8_to_hl(&mut self, e8: i8) -> u32 {
        let res = if e8 < 0 {
            let add = e8 as u8;
            let (res, _) = self.sp.overflowing_add(add as u16);
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

        self.write_reg16(Register16::HL, res);
        3
    }

    /// Load register HL into SP
    ///
    /// 2 cycles
    pub(super) fn ld_hl_to_sp(&mut self) -> u32 {
        self.sp = self.reg16(Register16::HL);
        2
    }

    /// For completeness
    ///
    /// 1 cycle
    pub(super) fn nop(&self) -> u32 { 1 }

    /// Calculate the bitwise or between register A and the specified register and
    /// store the result in register A.
    ///
    /// 1 cycle
    pub(super) fn or_r8(&mut self, reg: Register8) -> u32 {
        self.or(self.reg(reg));
        1
    }

    /// Calculate the bitwise or between register A and the byte pointed to by HL
    /// and store the result in register A.
    ///
    /// 2 cycles
    pub(super) fn or_hl(&mut self) -> u32 {
        self.or(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Calculate the bitwise or between register A and the specified byte and
    /// store the result in register A.
    ///
    /// 2 cycles
    pub(super) fn or(&mut self, n8: u8) -> u32 {
        let res = self.a_reg() | n8;
        self.set_zero_bit(res == 0);
        self.set_half_carry_bit(false);
        self.set_carry_bit(false);
        self.set_negative_bit(false);
        *self.a_reg_mut() = res;
        2
    }

    /// Pop register AF from the stack
    ///
    /// 3 cycles
    pub(super) fn pop_af(&mut self) -> u32 {
        // Flags should automatically be set, by loading this byte
        self.ld_const16addr_to_r8(self.sp, Register8::F);
        self.inc_sp();
        self.ld_const16addr_to_r8(self.sp, Register8::A);
        self.inc_sp();
        3
    }

    /// Pop specified register from stack
    ///
    /// 3 cycles
    pub(super) fn pop_r16(&mut self, reg: Register16) -> u32 {
        let (low_reg, high_reg) = reg.split();
        self.ld_const16addr_to_r8(self.sp, low_reg);
        self.inc_sp();
        self.ld_const16addr_to_r8(self.sp, high_reg);
        self.inc_sp();
        3
    }

    /// Push register AF into the stack.
    ///
    /// 4 cycles
    pub(super) fn push_af(&mut self) -> u32 {
        self.dec_sp();
        self.ld_r8_to_const16addr(Register8::A, self.sp);
        self.dec_sp();
        // Should automatically handle pushing the flags
        self.ld_r8_to_const16addr(Register8::F, self.sp);
        4
    }

    /// Push the specified register into the stack
    ///
    /// 4 cycles
    pub(super) fn push_r16(&mut self, reg: Register16) -> u32 {
        let (low_reg, high_reg) = reg.split();
        self.dec_sp();
        self.ld_r8_to_const16addr(high_reg, self.sp);
        self.dec_sp();
        self.ld_r8_to_const16addr(low_reg, self.sp);
        4
    }

    /// Push the specified 16 bit value into the stack
    ///
    /// This does not exist on the GB classic
    fn push_n16(&mut self, n16: u16) {
        let bytes = n16.to_le_bytes();
        self.dec_sp();
        // Higher byte first
        self.ld_const8_to_const16addr(bytes[1], self.sp);
        self.dec_sp();
        // Lower byte second
        self.ld_const8_to_const16addr(bytes[0], self.sp);
    }

    /// Set the specified bit of the register to 0
    ///
    /// 2 cycles
    pub(super) fn res_r8(&mut self, reg: Register8, bit: u8) -> u32 {
        *self.reg_mut(reg) &= !(1 << bit);
        2
    }

    /// Set the specified bit of the byte pointed to by HL to 0
    ///
    /// 4 cycles
    pub(super) fn res_hl(&mut self, bit: u8) -> u32 {
        let mut val = self.mmu.read_8(self.reg16(Register16::HL));
        val &= !(1 << bit);
        self.mmu.write_8(self.reg16(Register16::HL), val);
        4
    }

    /// Return from subroutine. This is basically POP PC, if it had existed.
    ///
    /// 4 cycles
    pub(super) fn ret(&mut self) -> u32 {
        let low_byte = self.mmu.read_8(self.sp);
        self.inc_sp();
        let high_byte = self.mmu.read_8(self.sp);
        self.inc_sp();
        self.pc = u16::from_le_bytes([low_byte, high_byte]);
        4
    }

    /// Return from subroutine if condition is met.
    ///
    /// 5 cycles if condition is met.
    /// 2 cycles if condition is not met.
    pub(super) fn ret_cc(&mut self, cc: Condition) -> u32 {
        if self.check_condition(cc) {
            self.ret();
            5
        } else {
            2
        }
    }

    /// Enable interrupts and return from subroutine.
    /// This IMMEDIATELY enables interrupts after the instruction in contrast to EI
    ///
    /// 4 cycles
    pub(super) fn reti(&mut self) -> u32 {
        self.ei();
        self.ret();
        4
    }

    /// Rotate the register left through the carry bit
    ///
    /// 2 cycles
    pub(super) fn rl(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.rl_helper(self.reg(reg));
        2
    }

    /// Rotate the byte pointed to by HL to the left through the carry bit
    ///
    /// 4 cycles
    pub(super) fn rl_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.rl_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    /// Rotate the A register to the left through the carry bit.
    /// The resulting flags are a bit different.
    ///
    /// 1 cycle
    pub(super) fn rla(&mut self) -> u32 {
        self.rl(Register8::A);
        self.set_zero_bit(false); // By definition
        1
    }

    /// A small helper to rotate the specified byte to the left through the carry bit
    fn rl_helper(&mut self, mut n8: u8) -> u8 {
        // Behavior (apparently)
        // Index
        // C
        // a
        // r
        // r
        // y
        // C 7 6 5 4 3 2 1 0 -> before
        // 7 6 5 4 3 2 1 0 C -> after
        // Check if the carry bit is set (as u8 for ease of use)
        let carry = if self.carry_bit() { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(check_bit(n8, 7));
        // Shift the register left by one
        n8 <<= 1;
        // OR the carry back in
        n8 |= carry;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Rotate the specified register
    ///
    /// 2 cycles
    pub(super) fn rlc(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.rlc_helper(self.reg(reg));
        2
    }

    /// Rotate the byte pointed to by HL
    ///
    /// 4 cycles
    pub(super) fn rlc_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.rlc_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    /// Rotate the A register to the left. The resulting flags are a bit different.
    ///
    /// 1 cycle
    pub(super) fn rlca(&mut self) -> u32 {
        self.rlc(Register8::A);
        self.set_zero_bit(false); // By definition
        1
    }

    /// Rotate the specified byte to the left
    fn rlc_helper(&mut self, mut n8: u8) -> u8{
        // Behavior (apparently)
        // Index
        // C
        // a
        // r
        // r
        // y
        // C 7 6 5 4 3 2 1 0 -> before
        // 7 6 5 4 3 2 1 0 7 -> after
        // Check if the carry bit is set (as u8 for ease of use)
        let truncated = if check_bit(n8, 7) { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(truncated != 0);
        // Shift the register left by one
        n8 <<= 1;
        // OR the carry back in
        n8 |= truncated;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Rotate the specified register to the right through the carry bit
    ///
    /// 2 cycles
    pub(super) fn rr(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.rr_helper(self.reg(reg));
        2
    }

    /// Rotate the byte pointed to by HL to the right throught the carry bit
    ///
    /// 4 cycles
    pub(super) fn rr_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.rr_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    /// Rotate the A register to the right through the carry bit.
    /// The resulting flags are a bit different.
    ///
    /// 1 cycle
    pub(super) fn rra(&mut self) -> u32 {
        self.rr(Register8::A);
        self.set_zero_bit(false); // By definition
        1
    }

    /// A helper to rotate the specified byte to the right
    fn rr_helper(&mut self, mut n8: u8) -> u8 {
        // Behavior (apparently)
        // Index
        //                 C
        //                 a
        //                 r
        //                 r
        //                 y
        // 7 6 5 4 3 2 1 0 C -> before
        // C 7 6 5 4 3 2 1 0 -> after
        // Check if the carry bit is set (as u8 for ease of use)
        let carry = if self.carry_bit() { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(check_bit(n8, 0));
        // Shift the register left by one
        n8 >>= 1;
        // OR the carry back in
        n8 |= carry << 7;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Rotate the specified register to the right
    ///
    /// 2 cycles
    pub(super) fn rrc(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.rrc_helper(self.reg(reg));
        2
    }

    /// Rotate the byte pointed to by HL to the right
    ///
    /// 4 cycles
    pub(super) fn rrc_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.rrc_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    /// Rotate the A register to the right. The resulting flags are a bit different
    ///
    /// 1 cycle
    pub(super) fn rrca(&mut self) -> u32 {
        self.rrc(Register8::A);
        self.set_zero_bit(false); // By definition
        1
    }

    /// Rotate the specified byte to the right
    fn rrc_helper(&mut self, mut n8: u8) -> u8 {
        // Behavior (apparently)
        // Index
        // C
        // a
        // r
        // r
        // y
        // C 7 6 5 4 3 2 1 0 -> before
        // 7 6 5 4 3 2 1 0 7 -> after
        // Check if the carry bit is set (as u8 for ease of use)
        let truncated = if check_bit(n8, 0) { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(truncated != 0);
        // Shift the register right by one
        n8 >>= 1;
        // OR the carry back in
        n8 |= truncated << 7;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Call the address associated with the reset vector. This is faster than a normal call
    ///
    /// 4 cycles
    pub(super) fn rst(&mut self, vec: ResetVec) -> u32 {
        self.push_n16(self.pc);
        self.pc = match vec {
            ResetVec::Vec1 => 0x00,
            ResetVec::Vec2 => 0x08,
            ResetVec::Vec3 => 0x10,
            ResetVec::Vec4 => 0x18,
            ResetVec::Vec5 => 0x20,
            ResetVec::Vec6 => 0x28,
            ResetVec::Vec7 => 0x30,
            ResetVec::Vec8 => 0x38,
        };
        4
    }

    /// Subtract the value in the specified register + the carry from the A register
    ///
    /// 1 cycle
    pub(super) fn sbc_r8(&mut self, reg: Register8) -> u32 {
        self.sbc(self.reg(reg));
        1
    }

    /// Subtract the value in the byte pointed to by HL + the carry from the A register
    ///
    /// 2 cycles
    pub(super) fn sbc_hl(&mut self) -> u32 {
        self.sbc(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Subtract the byte + the carry from the A register
    ///
    /// 2 cycles
    pub(super) fn sbc(&mut self, n8: u8) -> u32 {
        let carry = if self.carry_bit() { 1u8 } else { 0u8 };
        let result = self.a_reg() as i16 - n8 as i16 - carry as i16;

        self.set_carry_bit(result < 0);
        let result = result as u8; // Same behavior as static_cast in C++
        self.set_half_carry_bit(
            ((self.a_reg() & 0xF) as i16 - (n8 & 0xF) as i16 - carry as i16) < 0
        );
        self.set_negative_bit(true); // By definition
        self.set_zero_bit(result == 0);
        *self.a_reg_mut() = result;
        2
    }

    /// Set the carry flag
    ///
    /// 1 cycle
    pub(super) fn scf(&mut self) -> u32 {
        self.set_carry_bit(true); // By definition
        self.set_negative_bit(false); // By definition
        self.set_half_carry_bit(false); // By definition
        1
    }

    /// Set the specified bit in the specified register to high
    ///
    /// 2 cycles
    pub(super) fn set_r8(&mut self, reg: Register8, bit: u8) -> u32 {
        // TODO check if set methods work correctly
        *self.reg_mut(reg) |= 1 << bit;
        2
    }

    /// Set the specified bit in the byte pointed to by HL to high
    ///
    /// 4 cycles
    pub(super) fn set_hl(&mut self, bit: u8) -> u32 {
        let mut read = self.mmu.read_8(self.reg16(Register16::HL));
        read |= 1 << bit;
        self.mmu.write_8(self.reg16(Register16::HL), read);
        4
    }

    /// Shift the specified register to the left arithmetically
    ///
    /// 2 cycles
    pub(super) fn sla(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.sla_helper(self.reg(reg));
        2
    }

    /// Shift the byte pointed to by HL to the left arithmetically
    ///
    /// 4 cycles
    pub(super) fn sla_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.sla_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    /// A helper for shifting left arithmetically
    fn sla_helper(&mut self, mut n8: u8) -> u8 {
        // Behavior (apparently)
        // Index
        // C
        // a
        // r
        // r
        // y
        // C 7 6 5 4 3 2 1 0 -> before
        // 7 6 5 4 3 2 1 0 0 -> after
        // Check if the 0th bit is set.
        let truncated = if check_bit(n8, 0) { 1u8 } else { 0u8 };
        // Check if the last bit is set. This will be put into the carry flag
        let last = if check_bit(n8, 7) { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(last != 0);
        // Shift the register left by one
        n8 <<= 1;
        // OR the carry back in
        n8 |= truncated;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Shift the specified register to the right arithmetically
    ///
    /// 2 cycles
    pub(super) fn sra(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.sra_helper(self.reg(reg));
        2
    }

    /// Shift the byte pointed to by HL to the right arithmetically
    ///
    /// 4 cycles
    pub(super) fn sra_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.sra_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    fn sra_helper(&mut self, mut n8: u8) -> u8 {
        // Behavior (apparently)
        // Index
        //                 C
        //                 a
        //                 r
        //                 r
        //                 y
        // 7 6 5 4 3 2 1 0 C -> before
        // 7 7 6 5 4 3 2 1 0 -> after
        // Check if the 0th bit is set.
        let first = if check_bit(n8, 0) { 1u8 } else { 0u8 };
        // Check if the last bit is set. This will be put into the carry flag
        let last = if check_bit(n8, 7) { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(first != 0);
        // Shift the register left by one
        n8 >>= 1;
        // OR the last digit back in
        n8 |= last << 7;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Shift specified register to the right logically
    ///
    /// 2 cycles
    pub(super) fn srl(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.srl_helper(self.reg(reg));
        2
    }

    /// Shift the byte pointed to by HL to the right logically
    ///
    /// 4 cycles
    pub(super) fn srl_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.srl_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    fn srl_helper(&mut self, mut n8: u8) -> u8 {
        // Behavior (apparently)
        // Index
        //                 C
        //                 a
        //                 r
        //                 r
        //                 y
        // 7 6 5 4 3 2 1 0 C -> before
        // 0 7 6 5 4 3 2 1 0 -> after
        // Check if the 0th bit is set.
        let first = if check_bit(n8, 0) { 1u8 } else { 0u8 };
        // Set the carry bit according to the seventh bit of the register
        self.set_carry_bit(first != 0);
        // Shift the register left by one
        n8 >>= 1;
        // OR the last digit back in
        n8 |= first << 7;
        if n8 == 0 {
            self.set_zero_bit(true);
        }
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        n8
    }

    /// Enter CPU very low power mode
    ///
    /// - cycles
    pub(super) fn stop(&mut self) -> u32 {
        self.stopped = true;
        1
    }

    /// Subtract the value in the specified register from the A register
    ///
    /// 1 cycle
    pub(super) fn sub_r8(&mut self, reg: Register8) -> u32 {
        self.sub(self.reg(reg));
        1
    }

    /// Subtract the value from the byte pointed to by HL from the A register
    ///
    /// 2 cycles
    pub(super) fn sub_hl(&mut self) -> u32 {
        self.sub(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Subtract the specified value from the A register
    ///
    /// 2 cycles
    pub(super) fn sub(&mut self, n8: u8) -> u32 {
        let a = self.a_reg();
        self.set_zero_bit(a == n8); // Result is only zero, if A == n8
        self.set_negative_bit(true); // By definition
        // Result of lower nibble would have to borrow
        self.set_half_carry_bit((n8 & 0xF) > (a & 0xF));
        self.set_carry_bit(n8 > a); // Result would have to borrow
        *self.reg_mut(Register8::A) = a.overflowing_sub(n8).0;
        2
    }

    /// Swap the higher and lower 4 bits in the specified register
    ///
    /// 2 cycles
    pub(super) fn swap(&mut self, reg: Register8) -> u32 {
        *self.reg_mut(reg) = self.swap_helper(self.reg(reg));
        2
    }

    /// Swap the higher and lower 4 bits in the byte pointed to by HL
    ///
    /// 4 cycles
    pub(super) fn swap_hl(&mut self) -> u32 {
        let val = self.mmu.read_8(
            self.reg16(Register16::HL)
        );
        let val = self.swap_helper(val);
        let reg_val = self.reg16(Register16::HL);
        self.mmu.write_8(reg_val, val);
        4
    }

    /// Swap the lower and higher 4 bits, set flags as expected and return the result
    fn swap_helper(&mut self, n8: u8) -> u8 {
        let lower = n8 & 0x0F;
        let higher = n8 & 0xF0;
        let res = (lower << 4) | (higher >> 4);
        self.set_zero_bit(res == 0);
        self.set_negative_bit(false); // By definition
        self.set_half_carry_bit(false); // By definition
        self.set_carry_bit(false); // By definition
        res
    }

    /// Bitwise XOR between the value in r8 and the A register. Store the result in the A
    /// register.
    ///
    /// 1 cycle
    pub(super) fn xor_r8(&mut self, reg: Register8) -> u32 {
        self.xor(self.reg(reg));
        1
    }

    /// Bitwise XOR between the byte pointed to by HL and the A register. Store the result in the
    /// A register.
    ///
    /// 2 cycles
    pub(super) fn xor_hl(&mut self) -> u32 {
        self.xor(self.mmu.read_8(self.reg16(Register16::HL)));
        2
    }

    /// Bitwise XOR between the value in n8 and the A register. Store the result in the A register.
    ///
    /// 2 cycles
    pub(super) fn xor(&mut self, n8: u8) -> u32 {
        let res = self.reg(Register8::A) ^ n8;
        self.set_zero_bit(res == 0);
        self.set_carry_bit(false); // By definition
        self.set_half_carry_bit(false); // By definition
        self.set_negative_bit(false); // By definition
        2
    }
}