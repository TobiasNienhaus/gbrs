use super::*;
use super::instructions::*;
use std::net::Shutdown::Read;

impl Cpu {
    pub fn tick(&mut self) -> u32 {
        if !self.is_running() {
            return 1;
        }
        if self.pc == 0x0392 {
            println!("This is happening!");
        } else if self.pc >= 0x0380 && self.pc <= 0x0392 {
            self.dump_flags();
            println!("This is reached")
        }
        let instruction = self.read_u8();
        // println!("Instruction: {:#04X} Program counter: {:#06X}", instruction, self.pc);
        // println!("Instruction: {:#04X} PC: {:#06X} LY: {:#04X}", instruction, self.pc, self.mmu.read_ly());

        let cycle_count = match instruction {
            0x00 => self.nop(),
            0x01 => {
                let param = self.read_u16();
                self.ld_const16_to_r16(Register16::BC, param)
            },
            0x02 => self.ld_a_to_r16addr(Register16::BC),
            0x03 => self.inc_r16(Register16::BC),
            0x04 => self.inc_r8(Register8::B),
            0x05 => self.dec_r8(Register8::B),
            0x06 => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::B, param)
            },
            0x07 => self.rlca(),
            0x08 => {
                let param = self.read_u16();
                self.ld_sp_to_const16addr(param)
            },
            0x09 => self.add_r16_to_hl(Register16::BC),
            0x0A => self.ld_r16addr_to_a(Register16::BC),
            0x0B => self.dec_r16(Register16::BC),
            0x0C => self.inc_r8(Register8::C),
            0x0D => self.dec_r8(Register8::C),
            0x0E => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::C, param)
            },
            0x0F => self.rrca(),
            0x10 => {
                self.pc += 1; // STOP reads two bytes
                self.stop()
                // TODO I think this also has to pop off the next byte of the memory
                // -> increase program counter twice maybe?
            }
            0x11 => {
                let param = self.read_u16();
                self.ld_const16_to_r16(Register16::DE, param)
            },
            0x12 => self.ld_a_to_r16addr(Register16::DE),
            0x13 => self.inc_r16(Register16::DE),
            0x14 => self.inc_r8(Register8::D),
            0x15 => self.dec_r8(Register8::D),
            0x16 => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::D, param)
            },
            0x17 => self.rla(),
            0x18 => {
                let param = self.read_i8();
                self.jr(param)
            },
            0x19 => self.add_r16_to_hl(Register16::DE),
            0x1A => self.ld_r16addr_to_a(Register16::DE),
            0x1B => self.dec_r16(Register16::DE),
            0x1C => self.inc_r8(Register8::E),
            0x1D => self.dec_r8(Register8::E),
            0x1E => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::E, param)
            },
            0x1F => self.rra(),
            0x20 => {
                let param = self.read_i8();
                self.jr_cc(Condition::ZNotSet, param)
            },
            0x21 => {
                let param = self.read_u16();
                self.ld_const16_to_r16(Register16::HL, param)
            },
            0x22 => self.ld_a_to_hl_and_inc(),
            0x23 => self.inc_r16(Register16::HL),
            0x24 => self.inc_r8(Register8::H),
            0x25 => self.dec_r8(Register8::H),
            0x26 => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::H, param)
            },
            0x27 => self.daa(),
            0x28 => {
                let param = self.read_i8();
                self.jr_cc(Condition::ZSet, param)
            },
            0x29 => self.add_r16_to_hl(Register16::HL),
            0x2A => self.ld_hl_to_a_and_inc(),
            0x2B => self.dec_r16(Register16::HL),
            0x2C => self.inc_r8(Register8::L),
            0x2D => self.dec_r8(Register8::L),
            0x2E => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::L, param)
            },
            0x2F => self.cpl(),
            0x30 => {
                let param = self.read_i8();
                self.jr_cc(Condition::CNotSet, param)
            },
            0x31 => {
                let param = self.read_u16();
                self.ld_const16_to_sp(param)
            },
            0x32 => self.ld_a_to_hl_and_dec(),
            0x33 => self.inc_sp(),
            0x34 => self.inc_hl(),
            0x35 => self.dec_hl(),
            0x36 => {
                let param = self.read_u8();
                self.ld_const8_to_hl(param)
            },
            0x37 => self.scf(),
            0x38 => {
                let param = self.read_i8();
                self.jr_cc(Condition::CSet, param)
            },
            0x39 => self.add_sp_to_hl(),
            0x3A => self.ld_hl_to_a_and_dec(),
            0x3B => self.dec_sp(),
            0x3C => self.inc_r8(Register8::A),
            0x3D => self.dec_r8(Register8::A),
            0x3E => {
                let param = self.read_u8();
                self.ld_const8_to_r8(Register8::A, param)
            },
            0x3F => self.ccf(),
            0x40 => self.ld_r8_to_r8(Register8::B, Register8::B),
            0x41 => self.ld_r8_to_r8(Register8::B, Register8::C),
            0x42 => self.ld_r8_to_r8(Register8::B, Register8::D),
            0x43 => self.ld_r8_to_r8(Register8::B, Register8::E),
            0x44 => self.ld_r8_to_r8(Register8::B, Register8::H),
            0x45 => self.ld_r8_to_r8(Register8::B, Register8::L),
            0x46 => self.ld_hl_to_r8(Register8::B),
            0x47 => self.ld_r8_to_r8(Register8::B, Register8::A),
            0x48 => self.ld_r8_to_r8(Register8::C, Register8::B),
            0x49 => self.ld_r8_to_r8(Register8::C, Register8::C),
            0x4A => self.ld_r8_to_r8(Register8::C, Register8::D),
            0x4B => self.ld_r8_to_r8(Register8::C, Register8::E),
            0x4C => self.ld_r8_to_r8(Register8::C, Register8::H),
            0x4D => self.ld_r8_to_r8(Register8::C, Register8::L),
            0x4E => self.ld_hl_to_r8(Register8::C),
            0x4F => self.ld_r8_to_r8(Register8::C, Register8::A),
            0x50 => self.ld_r8_to_r8(Register8::D, Register8::B),
            0x51 => self.ld_r8_to_r8(Register8::D, Register8::C),
            0x52 => self.ld_r8_to_r8(Register8::D, Register8::D),
            0x53 => self.ld_r8_to_r8(Register8::D, Register8::E),
            0x54 => self.ld_r8_to_r8(Register8::D, Register8::H),
            0x55 => self.ld_r8_to_r8(Register8::D, Register8::L),
            0x56 => self.ld_hl_to_r8(Register8::D),
            0x57 => self.ld_r8_to_r8(Register8::D, Register8::A),
            0x58 => self.ld_r8_to_r8(Register8::E, Register8::B),
            0x59 => self.ld_r8_to_r8(Register8::E, Register8::C),
            0x5A => self.ld_r8_to_r8(Register8::E, Register8::D),
            0x5B => self.ld_r8_to_r8(Register8::E, Register8::E),
            0x5C => self.ld_r8_to_r8(Register8::E, Register8::H),
            0x5D => self.ld_r8_to_r8(Register8::E, Register8::L),
            0x5E => self.ld_hl_to_r8(Register8::E),
            0x5F => self.ld_r8_to_r8(Register8::E, Register8::A),
            0x60 => self.ld_r8_to_r8(Register8::H, Register8::B),
            0x61 => self.ld_r8_to_r8(Register8::H, Register8::C),
            0x62 => self.ld_r8_to_r8(Register8::H, Register8::D),
            0x63 => self.ld_r8_to_r8(Register8::H, Register8::E),
            0x64 => self.ld_r8_to_r8(Register8::H, Register8::H),
            0x65 => self.ld_r8_to_r8(Register8::H, Register8::L),
            0x66 => self.ld_hl_to_r8(Register8::H),
            0x67 => self.ld_r8_to_r8(Register8::H, Register8::A),
            0x68 => self.ld_r8_to_r8(Register8::L, Register8::B),
            0x69 => self.ld_r8_to_r8(Register8::L, Register8::C),
            0x6A => self.ld_r8_to_r8(Register8::L, Register8::D),
            0x6B => self.ld_r8_to_r8(Register8::L, Register8::E),
            0x6C => self.ld_r8_to_r8(Register8::L, Register8::H),
            0x6D => self.ld_r8_to_r8(Register8::L, Register8::L),
            0x6E => self.ld_hl_to_r8(Register8::L),
            0x6F => self.ld_r8_to_r8(Register8::L, Register8::A),
            0x70 => self.ld_r8_to_hl(Register8::B),
            0x71 => self.ld_r8_to_hl(Register8::C),
            0x72 => self.ld_r8_to_hl(Register8::D),
            0x73 => self.ld_r8_to_hl(Register8::E),
            0x74 => self.ld_r8_to_hl(Register8::H),
            0x75 => self.ld_r8_to_hl(Register8::L),
            0x76 => self.halt(), // TODO when interrupts are disabled, only skip one instruction
            0x77 => self.ld_r8_to_hl(Register8::A),
            0x78 => self.ld_r8_to_r8(Register8::A, Register8::B),
            0x79 => self.ld_r8_to_r8(Register8::A, Register8::C),
            0x7A => self.ld_r8_to_r8(Register8::A, Register8::D),
            0x7B => self.ld_r8_to_r8(Register8::A, Register8::E),
            0x7C => self.ld_r8_to_r8(Register8::A, Register8::H),
            0x7D => self.ld_r8_to_r8(Register8::A, Register8::L),
            0x7E => self.ld_hl_to_r8(Register8::A),
            0x7F => self.ld_r8_to_r8(Register8::A, Register8::A),
            0x80 => self.add_r8(Register8::B),
            0x81 => self.add_r8(Register8::C),
            0x82 => self.add_r8(Register8::D),
            0x83 => self.add_r8(Register8::E),
            0x84 => self.add_r8(Register8::H),
            0x85 => self.add_r8(Register8::L),
            0x86 => self.add_hl(),
            0x87 => self.add_r8(Register8::A),
            0x88 => self.adc_r8(Register8::B),
            0x89 => self.adc_r8(Register8::C),
            0x8A => self.adc_r8(Register8::D),
            0x8B => self.adc_r8(Register8::E),
            0x8C => self.adc_r8(Register8::H),
            0x8D => self.adc_r8(Register8::L),
            0x8E => self.adc_hl(),
            0x8F => self.adc_r8(Register8::A),
            0x90 => self.sub_r8(Register8::B),
            0x91 => self.sub_r8(Register8::C),
            0x92 => self.sub_r8(Register8::D),
            0x93 => self.sub_r8(Register8::E),
            0x94 => self.sub_r8(Register8::H),
            0x95 => self.sub_r8(Register8::L),
            0x96 => self.sub_hl(),
            0x97 => self.sub_r8(Register8::A),
            0x98 => self.sbc_r8(Register8::B),
            0x99 => self.sbc_r8(Register8::C),
            0x9A => self.sbc_r8(Register8::D),
            0x9B => self.sbc_r8(Register8::E),
            0x9C => self.sbc_r8(Register8::H),
            0x9D => self.sbc_r8(Register8::L),
            0x9E => self.sbc_hl(),
            0x9F => self.sbc_r8(Register8::A),
            0xA0 => self.and_r8(Register8::B),
            0xA1 => self.and_r8(Register8::C),
            0xA2 => self.and_r8(Register8::D),
            0xA3 => self.and_r8(Register8::E),
            0xA4 => self.and_r8(Register8::H),
            0xA5 => self.and_r8(Register8::L),
            0xA6 => self.and_hl(),
            0xA7 => self.and_r8(Register8::A),
            0xA8 => self.xor_r8(Register8::B),
            0xA9 => self.xor_r8(Register8::C),
            0xAA => self.xor_r8(Register8::D),
            0xAB => self.xor_r8(Register8::E),
            0xAC => self.xor_r8(Register8::H),
            0xAD => self.xor_r8(Register8::L),
            0xAE => self.xor_hl(),
            0xAF => self.xor_r8(Register8::A),
            0xB0 => self.or_r8(Register8::B),
            0xB1 => self.or_r8(Register8::C),
            0xB2 => self.or_r8(Register8::D),
            0xB3 => self.or_r8(Register8::E),
            0xB4 => self.or_r8(Register8::H),
            0xB5 => self.or_r8(Register8::L),
            0xB6 => self.or_hl(),
            0xB7 => self.or_r8(Register8::A),
            0xB8 => self.cp_r8(Register8::B),
            0xB9 => self.cp_r8(Register8::C),
            0xBA => self.cp_r8(Register8::D),
            0xBB => self.cp_r8(Register8::E),
            0xBC => self.cp_r8(Register8::H),
            0xBD => self.cp_r8(Register8::L),
            0xBE => self.cp_hl(),
            0xBF => self.cp_r8(Register8::A),
            0xC0 => self.ret_cc(Condition::ZNotSet),
            0xC1 => self.pop_r16(Register16::BC),
            0xC2 => {
                let param = self.read_u16();
                self.jp_cc(Condition::ZNotSet, param)
            },
            0xC3 => {
                let param = self.read_u16();
                self.jp(param)
            },
            0xC4 => {
                let param = self.read_u16();
                self.call_cc(Condition::ZNotSet, param)
            },
            0xC5 => self.push_r16(Register16::BC),
            0xC6 => {
                let param = self.read_u8();
                self.add(param)
            },
            0xC7 => self.rst(ResetVec::Vec1),
            0xC8 => self.ret_cc(Condition::ZSet),
            0xC9 => self.ret(),
            0xCA => {
                let param = self.read_u16();
                self.jp_cc(Condition::ZSet, param)
            }
            0xCB => self.execute_cb(), // TODO Should there be one cycle added?
            0xCC => {
                let param = self.read_u16();
                self.call_cc(Condition::ZSet, param)
            },
            0xCD => {
                let param = self.read_u16();
                self.call(param)
            },
            0xCE => {
                let param = self.read_u8();
                self.adc(param)
            },
            0xCF => self.rst(ResetVec::Vec2),
            0xD0 => self.ret_cc(Condition::CNotSet),
            0xD1 => self.pop_r16(Register16::DE),
            0xD2 => {
                let param = self.read_u16();
                self.jp_cc(Condition::CNotSet, param)
            },
            0xD4 => {
                let param = self.read_u16();
                self.call_cc(Condition::CNotSet, param)
            },
            0xD5 => self.push_r16(Register16::DE),
            0xD6 => {
                let param = self.read_u8();
                self.sub(param)
            },
            0xD7 => self.rst(ResetVec::Vec3),
            0xD8 => self.ret_cc(Condition::CSet),
            0xD9 => self.reti(),
            0xDA => {
                let param = self.read_u16();
                self.jp_cc(Condition::CSet, param)
            },
            0xDC => {
                let param = self.read_u16();
                self.call_cc(Condition::CSet, param)
            },
            0xDE => {
                let param = self.read_u8();
                self.sbc(param)
            },
            0xDF => self.rst(ResetVec::Vec4),
            0xE0 => {
                let param = self.read_u8();
                self.ldh_a_to_const16addr(0xFF00 + (param as u16))
            },
            0xE1 => self.pop_r16(Register16::HL),
            0xE2 => self.ldh_a_to_ff00_plus_c(),
            0xE5 => self.push_r16(Register16::HL),
            0xE6 => {
                let param = self.read_u8();
                self.and(param)
            },
            0xE7 => self.rst(ResetVec::Vec5),
            0xE8 => {
                let param = self.read_i8();
                self.add_e8_to_sp(param)
            },
            0xE9 => self.jp_hl(),
            0xEA => {
                let param = self.read_u16();
                self.ld_a_to_const16addr(param)
            },
            0xEE => {
                let param = self.read_u8();
                self.xor(param)
            },
            0xEF => self.rst(ResetVec::Vec6),
            0xF0 => {
                let param = self.read_u8();
                self.ldh_const16addr_to_a(0xFF00 + (param as u16))
            },
            0xF1 => self.pop_af(),
            0xF2 => self.ldh_ff00_plus_c_to_a(),
            0xF3 => self.di(),
            0xF5 => self.push_af(),
            0xF6 => {
                let param = self.read_u8();
                self.or(param)
            },
            0xF7 => self.rst(ResetVec::Vec7),
            0xF8 => {
                let param = self.read_i8();
                self.ld_sp_plus_e8_to_hl(param)
            },
            0xF9 => self.ld_hl_to_sp(),
            0xFA => {
                let param = self.read_u16();
                self.ld_const16addr_to_a(param)
            },
            0xFB => self.ei(),
            0xFE => {
                let param = self.read_u8();
                self.cp(param)
            },
            0xFF => self.rst(ResetVec::Vec8),
            _ => unreachable!("{:#04X} is not a valid instruction code", instruction)
        };
        cycle_count
    }

    fn execute_cb(&mut self) -> u32 {
        let instruction: u8 = self.read_u8();

        let cycle_count = match instruction {
            0x00 => self.rlc(Register8::B),
            0x01 => self.rlc(Register8::C),
            0x02 => self.rlc(Register8::D),
            0x03 => self.rlc(Register8::E),
            0x04 => self.rlc(Register8::H),
            0x05 => self.rlc(Register8::L),
            0x06 => self.rlc_hl(),
            0x07 => self.rlc(Register8::A),
            0x08 => self.rrc(Register8::B),
            0x09 => self.rrc(Register8::C),
            0x0A => self.rrc(Register8::D),
            0x0B => self.rrc(Register8::E),
            0x0C => self.rrc(Register8::H),
            0x0D => self.rrc(Register8::L),
            0x0E => self.rrc_hl(),
            0x0F => self.rrc(Register8::A),
            0x10 => self.rl(Register8::B),
            0x11 => self.rl(Register8::C),
            0x12 => self.rl(Register8::D),
            0x13 => self.rl(Register8::E),
            0x14 => self.rl(Register8::H),
            0x15 => self.rl(Register8::L),
            0x16 => self.rl_hl(),
            0x17 => self.rl(Register8::A),
            0x18 => self.rr(Register8::B),
            0x19 => self.rr(Register8::C),
            0x1A => self.rr(Register8::D),
            0x1B => self.rr(Register8::E),
            0x1C => self.rr(Register8::H),
            0x1D => self.rr(Register8::L),
            0x1E => self.rr_hl(),
            0x1F => self.rr(Register8::A),
            0x20 => self.sla(Register8::B),
            0x21 => self.sla(Register8::C),
            0x22 => self.sla(Register8::D),
            0x23 => self.sla(Register8::E),
            0x24 => self.sla(Register8::H),
            0x25 => self.sla(Register8::L),
            0x26 => self.sla_hl(),
            0x27 => self.sla(Register8::A),
            0x28 => self.sra(Register8::B),
            0x29 => self.sra(Register8::C),
            0x2A => self.sra(Register8::D),
            0x2B => self.sra(Register8::E),
            0x2C => self.sra(Register8::H),
            0x2D => self.sra(Register8::L),
            0x2E => self.sra_hl(),
            0x2F => self.sra(Register8::A),
            0x30 => self.swap(Register8::B),
            0x31 => self.swap(Register8::C),
            0x32 => self.swap(Register8::D),
            0x33 => self.swap(Register8::E),
            0x34 => self.swap(Register8::H),
            0x35 => self.swap(Register8::L),
            0x36 => self.swap_hl(),
            0x37 => self.swap(Register8::A),
            0x38 => self.srl(Register8::B),
            0x39 => self.srl(Register8::C),
            0x3A => self.srl(Register8::D),
            0x3B => self.srl(Register8::E),
            0x3C => self.srl(Register8::H),
            0x3D => self.srl(Register8::L),
            0x3E => self.srl_hl(),
            0x3F => self.srl(Register8::A),
            0x40 => self.bit_r8(Register8::B, 0),
            0x41 => self.bit_r8(Register8::C, 0),
            0x42 => self.bit_r8(Register8::D, 0),
            0x43 => self.bit_r8(Register8::E, 0),
            0x44 => self.bit_r8(Register8::H, 0),
            0x45 => self.bit_r8(Register8::L, 0),
            0x46 => self.bit_hl(0),
            0x47 => self.bit_r8(Register8::A, 0),
            0x48 => self.bit_r8(Register8::B, 1),
            0x49 => self.bit_r8(Register8::C, 1),
            0x4A => self.bit_r8(Register8::D, 1),
            0x4B => self.bit_r8(Register8::E, 1),
            0x4C => self.bit_r8(Register8::H, 1),
            0x4D => self.bit_r8(Register8::L, 1),
            0x4E => self.bit_hl(1),
            0x4F => self.bit_r8(Register8::A, 1),
            0x50 => self.bit_r8(Register8::B, 2),
            0x51 => self.bit_r8(Register8::C, 2),
            0x52 => self.bit_r8(Register8::D, 2),
            0x53 => self.bit_r8(Register8::E, 2),
            0x54 => self.bit_r8(Register8::H, 2),
            0x55 => self.bit_r8(Register8::L, 2),
            0x56 => self.bit_hl(2),
            0x57 => self.bit_r8(Register8::A, 2),
            0x58 => self.bit_r8(Register8::B, 3),
            0x59 => self.bit_r8(Register8::C, 3),
            0x5A => self.bit_r8(Register8::D, 3),
            0x5B => self.bit_r8(Register8::E, 3),
            0x5C => self.bit_r8(Register8::H, 3),
            0x5D => self.bit_r8(Register8::L, 3),
            0x5E => self.bit_hl(3),
            0x5F => self.bit_r8(Register8::A, 3),
            0x60 => self.bit_r8(Register8::B, 4),
            0x61 => self.bit_r8(Register8::C, 4),
            0x62 => self.bit_r8(Register8::D, 4),
            0x63 => self.bit_r8(Register8::E, 4),
            0x64 => self.bit_r8(Register8::H, 4),
            0x65 => self.bit_r8(Register8::L, 4),
            0x66 => self.bit_hl(4),
            0x67 => self.bit_r8(Register8::A, 4),
            0x68 => self.bit_r8(Register8::B, 5),
            0x69 => self.bit_r8(Register8::C, 5),
            0x6A => self.bit_r8(Register8::D, 5),
            0x6B => self.bit_r8(Register8::E, 5),
            0x6C => self.bit_r8(Register8::H, 5),
            0x6D => self.bit_r8(Register8::L, 5),
            0x6E => self.bit_hl(5),
            0x6F => self.bit_r8(Register8::A, 5),
            0x70 => self.bit_r8(Register8::B, 6),
            0x71 => self.bit_r8(Register8::C, 6),
            0x72 => self.bit_r8(Register8::D, 6),
            0x73 => self.bit_r8(Register8::E, 6),
            0x74 => self.bit_r8(Register8::H, 6),
            0x75 => self.bit_r8(Register8::L, 6),
            0x76 => self.bit_hl(6),
            0x77 => self.bit_r8(Register8::A, 6),
            0x78 => self.bit_r8(Register8::B, 7),
            0x79 => self.bit_r8(Register8::C, 7),
            0x7A => self.bit_r8(Register8::D, 7),
            0x7B => self.bit_r8(Register8::E, 7),
            0x7C => self.bit_r8(Register8::H, 7),
            0x7D => self.bit_r8(Register8::L, 7),
            0x7E => self.bit_hl(7),
            0x7F => self.bit_r8(Register8::A, 7),
            0x80 => self.res_r8(Register8::B, 0),
            0x81 => self.res_r8(Register8::C, 0),
            0x82 => self.res_r8(Register8::D, 0),
            0x83 => self.res_r8(Register8::E, 0),
            0x84 => self.res_r8(Register8::H, 0),
            0x85 => self.res_r8(Register8::L, 0),
            0x86 => self.res_hl(0),
            0x87 => self.res_r8(Register8::A, 0),
            0x88 => self.res_r8(Register8::B, 1),
            0x89 => self.res_r8(Register8::C, 1),
            0x8A => self.res_r8(Register8::D, 1),
            0x8B => self.res_r8(Register8::E, 1),
            0x8C => self.res_r8(Register8::H, 1),
            0x8D => self.res_r8(Register8::L, 1),
            0x8E => self.res_hl(1),
            0x8F => self.res_r8(Register8::A, 1),
            0x90 => self.res_r8(Register8::B, 2),
            0x91 => self.res_r8(Register8::C, 2),
            0x92 => self.res_r8(Register8::D, 2),
            0x93 => self.res_r8(Register8::E, 2),
            0x94 => self.res_r8(Register8::H, 2),
            0x95 => self.res_r8(Register8::L, 2),
            0x96 => self.res_hl(2),
            0x97 => self.res_r8(Register8::A, 2),
            0x98 => self.res_r8(Register8::B, 3),
            0x99 => self.res_r8(Register8::C, 3),
            0x9A => self.res_r8(Register8::D, 3),
            0x9B => self.res_r8(Register8::E, 3),
            0x9C => self.res_r8(Register8::H, 3),
            0x9D => self.res_r8(Register8::L, 3),
            0x9E => self.res_hl(3),
            0x9F => self.res_r8(Register8::A, 3),
            0xA0 => self.res_r8(Register8::B, 4),
            0xA1 => self.res_r8(Register8::C, 4),
            0xA2 => self.res_r8(Register8::D, 4),
            0xA3 => self.res_r8(Register8::E, 4),
            0xA4 => self.res_r8(Register8::H, 4),
            0xA5 => self.res_r8(Register8::L, 4),
            0xA6 => self.res_hl(4),
            0xA7 => self.res_r8(Register8::A, 4),
            0xA8 => self.res_r8(Register8::B, 5),
            0xA9 => self.res_r8(Register8::C, 5),
            0xAA => self.res_r8(Register8::D, 5),
            0xAB => self.res_r8(Register8::E, 5),
            0xAC => self.res_r8(Register8::H, 5),
            0xAD => self.res_r8(Register8::L, 5),
            0xAE => self.res_hl(5),
            0xAF => self.res_r8(Register8::A, 5),
            0xB0 => self.res_r8(Register8::B, 6),
            0xB1 => self.res_r8(Register8::C, 6),
            0xB2 => self.res_r8(Register8::D, 6),
            0xB3 => self.res_r8(Register8::E, 6),
            0xB4 => self.res_r8(Register8::H, 6),
            0xB5 => self.res_r8(Register8::L, 6),
            0xB6 => self.res_hl(6),
            0xB7 => self.res_r8(Register8::A, 6),
            0xB8 => self.res_r8(Register8::B, 7),
            0xB9 => self.res_r8(Register8::C, 7),
            0xBA => self.res_r8(Register8::D, 7),
            0xBB => self.res_r8(Register8::E, 7),
            0xBC => self.res_r8(Register8::H, 7),
            0xBD => self.res_r8(Register8::L, 7),
            0xBE => self.res_hl(7),
            0xBF => self.res_r8(Register8::A, 7),
            0xC0 => self.set_r8(Register8::B, 0),
            0xC1 => self.set_r8(Register8::C, 0),
            0xC2 => self.set_r8(Register8::D, 0),
            0xC3 => self.set_r8(Register8::E, 0),
            0xC4 => self.set_r8(Register8::H, 0),
            0xC5 => self.set_r8(Register8::L, 0),
            0xC6 => self.set_hl(0),
            0xC7 => self.set_r8(Register8::A, 0),
            0xC8 => self.set_r8(Register8::B, 1),
            0xC9 => self.set_r8(Register8::C, 1),
            0xCA => self.set_r8(Register8::D, 1),
            0xCB => self.set_r8(Register8::E, 1),
            0xCC => self.set_r8(Register8::H, 1),
            0xCD => self.set_r8(Register8::L, 1),
            0xCE => self.set_hl(1),
            0xCF => self.set_r8(Register8::A, 1),
            0xD0 => self.set_r8(Register8::B, 2),
            0xD1 => self.set_r8(Register8::C, 2),
            0xD2 => self.set_r8(Register8::D, 2),
            0xD3 => self.set_r8(Register8::E, 2),
            0xD4 => self.set_r8(Register8::H, 2),
            0xD5 => self.set_r8(Register8::L, 2),
            0xD6 => self.set_hl(2),
            0xD7 => self.set_r8(Register8::A, 2),
            0xD8 => self.set_r8(Register8::B, 3),
            0xD9 => self.set_r8(Register8::C, 3),
            0xDA => self.set_r8(Register8::D, 3),
            0xDB => self.set_r8(Register8::E, 3),
            0xDC => self.set_r8(Register8::H, 3),
            0xDD => self.set_r8(Register8::L, 3),
            0xDE => self.set_hl(3),
            0xDF => self.set_r8(Register8::A, 3),
            0xE0 => self.set_r8(Register8::B, 4),
            0xE1 => self.set_r8(Register8::C, 4),
            0xE2 => self.set_r8(Register8::D, 4),
            0xE3 => self.set_r8(Register8::E, 4),
            0xE4 => self.set_r8(Register8::H, 4),
            0xE5 => self.set_r8(Register8::L, 4),
            0xE6 => self.set_hl(4),
            0xE7 => self.set_r8(Register8::A, 4),
            0xE8 => self.set_r8(Register8::B, 5),
            0xE9 => self.set_r8(Register8::C, 5),
            0xEA => self.set_r8(Register8::D, 5),
            0xEB => self.set_r8(Register8::E, 5),
            0xEC => self.set_r8(Register8::H, 5),
            0xED => self.set_r8(Register8::L, 5),
            0xEE => self.set_hl(5),
            0xEF => self.set_r8(Register8::A, 5),
            0xF0 => self.set_r8(Register8::B, 6),
            0xF1 => self.set_r8(Register8::C, 6),
            0xF2 => self.set_r8(Register8::D, 6),
            0xF3 => self.set_r8(Register8::E, 6),
            0xF4 => self.set_r8(Register8::H, 6),
            0xF5 => self.set_r8(Register8::L, 6),
            0xF6 => self.set_hl(6),
            0xF7 => self.set_r8(Register8::A, 6),
            0xF8 => self.set_r8(Register8::B, 7),
            0xF9 => self.set_r8(Register8::C, 7),
            0xFA => self.set_r8(Register8::D, 7),
            0xFB => self.set_r8(Register8::E, 7),
            0xFC => self.set_r8(Register8::H, 7),
            0xFD => self.set_r8(Register8::L, 7),
            0xFE => self.set_hl(7),
            0xFF => self.set_r8(Register8::A, 7),
            _ => unreachable!("{:#04X} is not a valid CB instruction code", instruction)
        };
        cycle_count
    }
}