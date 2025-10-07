use std::fmt::{Display, Formatter};
use crate::game_boy::cpu::{Cpu, Register16, Register8};
use crate::game_boy::InstructionInformation;

#[derive(Copy, Clone)]
pub struct DebugStackInfo {
    bc: u16,
    de: u16,
    hl: u16,
    af: u16,
    sp: u16,
    pc: u16
}

impl DebugStackInfo {
    pub fn pc(&self) -> u16 { self.pc }
}

impl Display for DebugStackInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BC={:04X} DE={:04X} HL={:04X} AF={:04X} SP={:04X} PC={:04X}",
            self.bc,
            self.de,
            self.hl,
            self.af,
            self.sp,
            self.pc
        )
    }
}

impl Cpu {
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn peek_instruction(&self) -> u8 {
        self.peek_u8()
    }

    pub fn peek_data(&self) -> [Option<u8>; 4] {
        let mut res: [Option<u8>; 4] = [None; 4];
        for i in 1..=4 {
            if u16::MAX - i > self.pc {
                res[(i - 1) as usize] = Some(self.mmu.read_8(self.pc + i))
            }
        }
        res
    }

    pub fn debug_stack_info(&self) -> DebugStackInfo {
        DebugStackInfo {
            bc: self.reg16(Register16::BC),
            de: self.reg16(Register16::DE),
            hl: self.reg16(Register16::HL),
            af: u16::from_le_bytes([self.reg(Register8::F), self.reg(Register8::A)]),
            pc: self.pc,
            sp: self.sp
        }
    }
}

pub fn pretty_instruction(info: &InstructionInformation, cpu: &Cpu) -> String {

    let n16 = info.data[0].zip(info.data[1]).map(|(a, b)| u16::from_le_bytes([a, b])).map_or(
        // The value to return if the Option is None
        "INVALID".to_string(),
        // The closure to run if the Option is Some(value)
        |value| format!("{:04x}", value)
    );

    let n8 = info.data[0].map(|a| u8::from_le_bytes([a])).map_or(
        // The value to return if the Option is None
        "INVALID".to_string(),
        // The closure to run if the Option is Some(value)
        |value| format!("{:02x}", value)
    );

    let e8 = info.data[0].map(|a| i8::from_le_bytes([a])).map_or(
        // The value to return if the Option is None
        "INVALID".to_string(),
        // The closure to run if the Option is Some(value)
        |value| format!("{:02x} ({})", value, value)
    );

    match info.instruction() {
        0x1 => format!("LD BC {}", n16),
        0x6 => format!("LD B, {}", n8),
        0x8 => format!("LD [{}], SP", n16),
        0xE => format!("LD C, {}", n8),
        0x10 => format!("STOP {}", n8),
        0x11 => format!("LD DE, {}", n16),
        0x16 => format!("LD D, {}", n8),
        0x18 => format!("JR {}", e8),
        0x1E => format!("LD, E, {}", n8),
        0x20 => format!("JR, NZ, {}", e8),
        0x21 => format!("LD HL, {}", n16),
        0x26 => format!("LD H, {}", n8),
        0x28 => format!("JR Z, {}", e8),
        0x2e => format!("LD L, {}", n8),
        0x31 => format!("LD SP, {}", n16),
        0x36 => format!("LD [HL], {}", n8),
        0x38 => format!("JR C, {}", e8),
        0x3e => format!("LD A, {}", n8),
        0xc2 => format!("JP NZ, {}", n16),
        0xc3 => format!("JP {}", n16),
        0xc4 => format!("CALL NZ, {}", n16),
        0xc6 => format!("ADD A, {}", n8),
        0xca => format!("JP Z, {}", n16),
        0xcc => format!("CALL Z, {}", n16),
        0xcd => format!("CALL {}", n16),
        0xce => format!("ADC A, {}", n8),
        0xd2 => format!("JP NC, {}", n16),
        0xd4 => format!("CALL NC, {}", n16),
        0xd6 => format!("SUB A, {}", n8),
        0xda => format!("JP C, {}", n16),
        0xdc => format!("CALL C, {}", n16),
        0xde => format!("SBC A, {}", n8),
        0xe0 => format!("LDH [ff00+{}], A", n8),
        0xe6 => format!("AND A, {}", n8),
        0xe8 => format!("ADD SP, {}", e8),
        0xea => format!("LD [{}], A", n16),
        0xee => format!("XOR A, {}", n8),
        0xf0 => format!("LDH A, [ff00+{}]", n8),
        0xf6 => format!("OR A, {}", n8),
        0xf8 => format!("LD HL, SP + {}", e8),
        0xfa => format!("LD A, [{}]", n16),
        0xfe => format!("CP A, {}", n8),
        i => ins_name(i, info.data[0]).to_owned()
    }
}

pub fn ins_name(ins: u8, next: Option<u8>) -> &'static str {
    match ins {
        0x0 => "NOP",
        0x1 => "LD BC n16",
        0x2 => "LD [BC], A",
        0x3 => "INC BC",
        0x4 => "INC B",
        0x5 => "DEC B",
        0x6 => "LD B, n8",
        0x7 => "RLCA",
        0x8 => "LD [a16], SP",
        0x9 => "ADD HL, BC",
        0xA => "LD A, [BC]",
        0xB => "DEC BC",
        0xC => "INC C",
        0xD => "DEC C",
        0xE => "LD C, n8",
        0xF => "RRCA",
        0x10 => "STOP n8",
        0x11 => "LD DE, n16",
        0x12 => "LD [DE], A",
        0x13 => "INC DE",
        0x14 => "INC D",
        0x15 => "DEC D",
        0x16 => "LD D, n8",
        0x17 => "RLA",
        0x18 => "JR e8",
        0x19 => "ADD HL, DE",
        0x1A => "LD A, [DE]",
        0x1B => "DEC DE",
        0x1C => "INC E",
        0x1D => "DEC E",
        0x1E => "LD, E, n8",
        0x1F => "RRA",
        0x20 => "JR, NZ, e8",
        0x21 => "LD HL, n16",
        0x22 => "LD [HL+], A",
        0x23 => "INC HL",
        0x24 => "INC H",
        0x25 => "DEC H",
        0x26 => "LD H, n8",
        0x27 => "DAA",
        0x28 => "JR Z, e8",
        0x29 => "ADD HL, HL",
        0x2a => "LD A, [HL+]",
        0x2b => "DEC HL",
        0x2c => "INC L",
        0x2d => "DEC L",
        0x2e => "LD L, n8",
        0x2f => "CPL",
        0x30 => "JR B, B",
        0x31 => "LD SP, n16",
        0x32 => "LD [HL-], A",
        0x33 => "INC SP",
        0x34 => "INC [HL]",
        0x35 => "DEC [HL]",
        0x36 => "LD [HL], n8",
        0x37 => "SCF",
        0x38 => "JR C, e8",
        0x39 => "ADD HL, SP",
        0x3a => "LD A, [HL-]",
        0x3b => "DEC SP",
        0x3c => "INC A",
        0x3d => "DEC A",
        0x3e => "LD A, n8",
        0x3f => "CCF",
        0x40 => "LD B, B",
        0x41 => "LD B, C",
        0x42 => "LD B, D",
        0x43 => "LD B, E",
        0x44 => "LD B, H",
        0x45 => "LD B, L",
        0x46 => "LD B, [HL]",
        0x47 => "LD B, A",
        0x48 => "LD C, B",
        0x49 => "LD C, C",
        0x4a => "LD C, D",
        0x4b => "LD C, E",
        0x4c => "LD C, F",
        0x4d => "LD C, G",
        0x4e => "LD C, [HL]",
        0x4f => "LD C, A",
        0x50 => "LD D, B",
        0x51 => "LD D, C",
        0x52 => "LD D, D",
        0x53 => "LD D, E",
        0x54 => "LD D, H",
        0x55 => "LD D, L",
        0x56 => "LD D, [HL]",
        0x57 => "LD D, A",
        0x58 => "LD E, B",
        0x59 => "LD E, C",
        0x5a => "LD E, D",
        0x5b => "LD E, E",
        0x5c => "LD E, F",
        0x5d => "LD E, G",
        0x5e => "LD E, [HL]",
        0x5f => "LD E, A",
        0x60 => "LD H, B",
        0x61 => "LD H, C",
        0x62 => "LD H, D",
        0x63 => "LD H, E",
        0x64 => "LD H, H",
        0x65 => "LD H, L",
        0x66 => "LD H, [HL]",
        0x67 => "LD H, A",
        0x68 => "LD L, B",
        0x69 => "LD L, C",
        0x6a => "LD L, D",
        0x6b => "LD L, E",
        0x6c => "LD L, F",
        0x6d => "LD L, G",
        0x6e => "LD L, [HL]",
        0x6f => "LD L, A",
        0x70 => "LD [HL], B",
        0x71 => "LD [HL], C",
        0x72 => "LD [HL], D",
        0x73 => "LD [HL], E",
        0x74 => "LD [HL], H",
        0x75 => "LD [HL], L",
        0x76 => "LD [HL], [HL]",
        0x77 => "LD [HL], A",
        0x78 => "LD A, B",
        0x79 => "LD A, C",
        0x7a => "LD A, D",
        0x7b => "LD A, E",
        0x7c => "LD A, F",
        0x7d => "LD A, G",
        0x7e => "LD A, [HL]",
        0x7f => "LD A, A",
        0x80 => "ADD A, B",
        0x81 => "ADD A, C",
        0x82 => "ADD A, D",
        0x83 => "ADD A, E",
        0x84 => "ADD A, H",
        0x85 => "ADD A, L",
        0x86 => "ADD A, [HL]",
        0x87 => "ADD A, A",
        0x88 => "ADC A, B",
        0x89 => "ADC A, C",
        0x8a => "ADC A, D",
        0x8b => "ADC A, E",
        0x8c => "ADC A, H",
        0x8d => "ADC A, L",
        0x8e => "ADC A, [HL]",
        0x8f => "ADC A, A",
        0x90 => "SUB A, B",
        0x91 => "SUB A, C",
        0x92 => "SUB A, D",
        0x93 => "SUB A, E",
        0x94 => "SUB A, H",
        0x95 => "SUB A, L",
        0x96 => "SUB A, [HL]",
        0x97 => "SUB A, A",
        0x98 => "SBC A, B",
        0x99 => "SBC A, C",
        0x9a => "SBC A, D",
        0x9b => "SBC A, E",
        0x9c => "SBC A, H",
        0x9d => "SBC A, L",
        0x9e => "SBC A, [HL]",
        0x9f => "SBC A, A",
        0xa0 => "AND A, B",
        0xa1 => "AND A, C",
        0xa2 => "AND A, D",
        0xa3 => "AND A, E",
        0xa4 => "AND A, H",
        0xa5 => "AND A, L",
        0xa6 => "AND A, [HL]",
        0xa7 => "AND A, A",
        0xa8 => "XOR A, B",
        0xa9 => "XOR A, C",
        0xaa => "XOR A, D",
        0xab => "XOR A, E",
        0xac => "XOR A, H",
        0xad => "XOR A, L",
        0xae => "XOR A, [HL]",
        0xaf => "XOR A, A",
        0xb0 => "OR A, B",
        0xb1 => "OR A, C",
        0xb2 => "OR A, D",
        0xb3 => "OR A, E",
        0xb4 => "OR A, H",
        0xb5 => "OR A, L",
        0xb6 => "OR A, [HL]",
        0xb7 => "OR A, A",
        0xb8 => "CP A, B",
        0xb9 => "CP A, C",
        0xba => "CP A, D",
        0xbb => "CP A, E",
        0xbc => "CP A, H",
        0xbd => "CP A, L",
        0xbe => "CP A, [HL]",
        0xbf => "CP A, A",
        0xc0 => "RET NZ",
        0xc1 => "POP BC",
        0xc2 => "JP NZ, a16",
        0xc3 => "JP a16",
        0xc4 => "CALL NZ, a16",
        0xc5 => "PUSH BC",
        0xc6 => "ADD A, n8",
        0xc7 => "RST $00",
        0xc8 => "RET Z",
        0xc9 => "RET",
        0xca => "JP Z, a16",
        0xcb => prefixed(next),
        0xcc => "CALL Z, a16",
        0xcd => "CALL a16",
        0xce => "ADC A, n8",
        0xcf => "RST $08",
        0xd0 => "RET NC",
        0xd1 => "POP DE",
        0xd2 => "JP NC, a16",
        0xd4 => "CALL NC, a16",
        0xd5 => "PUSH DE",
        0xd6 => "SUB A, n8",
        0xd7 => "RST $10",
        0xd8 => "RET C",
        0xd9 => "RETI",
        0xda => "JP C, a16",
        0xdc => "CALL C, a16",
        0xde => "SBC A, n8",
        0xdf => "RST $18",
        0xe0 => "LDH [a8], A",
        0xe1 => "POP HL",
        0xe2 => "LD [C], A",
        0xe5 => "PUSH HL",
        0xe6 => "AND A, n8",
        0xe7 => "RST $20",
        0xe8 => "ADD SP, e8",
        0xe9 => "JP HL",
        0xea => "LD [a16], A",
        0xee => "XOR A, n8",
        0xef => "RST $28",
        0xf0 => "LDH A, [a8]",
        0xf1 => "POP AF",
        0xf2 => "LD A, [C]",
        0xf3 => "DI (disable interrupts)",
        0xf5 => "PUSH AF",
        0xf6 => "OR A, n8",
        0xf7 => "RST $30",
        0xf8 => "LD HL, SP + e8",
        0xf9 => "LD SP, HL",
        0xfa => "LD A, [a16]",
        0xfb => "EI (enable interrupts)",
        0xfe => "CP A, n8",
        0xff => "RST $38",
        _ => "INVALID"
    }
}

fn prefixed(byte: Option<u8>) -> &'static str {
    if let Some(byte) = byte {
        match byte {
            0x00..=0x07 => "RLC",
            0x08..=0x0F => "RRC",
            0x10..=0x17 => "RL",
            0x18..=0x1F => "RR",
            0x20..=0x27 => "SLA",
            0x28..=0x2F => "SRA",
            0x30..=0x37 => "SWAP",
            0x38..=0x3F => "SRL",
            0x40..=0x47 => "BIT 0",
            0x48..=0x4F => "BIT 1",
            0x50..=0x57 => "BIT 2",
            0x58..=0x5F => "BIT 3",
            0x60..=0x67 => "BIT 4",
            0x68..=0x6F => "BIT 5",
            0x70..=0x77 => "BIT 6",
            0x78..=0x7F => "BIT 7",
            0x80..=0x87 => "RES 0",
            0x88..=0x8F => "RES 1",
            0x90..=0x97 => "RES 2",
            0x98..=0x9F => "RES 3",
            0xA0..=0xA7 => "RES 4",
            0xA8..=0xAF => "RES 5",
            0xB0..=0xB7 => "RES 6",
            0xB8..=0xBF => "RES 7",
            0xC0..=0xC7 => "SET 0",
            0xC8..=0xCF => "SET 1",
            0xD0..=0xD7 => "SET 2",
            0xD8..=0xDF => "SET 3",
            0xE0..=0xE7 => "SET 4",
            0xE8..=0xEF => "SET 5",
            0xF0..=0xF7 => "SET 6",
            0xF8..=0xFF => "SET 7",
            _ => "INVALID"
        }
    } else {
        "INVALID"
    }
}
