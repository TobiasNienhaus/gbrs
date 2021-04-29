
pub(super) struct Cpu {
    a_reg: u8,
    b_reg: u8,
    c_reg: u8,
    d_reg: u8,
    e_reg: u8,
    h_reg: u8,
    l_reg: u8,
    flag_reg: u8,
    pc: u16,
    sp: u16
}

impl Cpu {
    pub(super) fn write_af(&mut self, val: u16) {
        let af = val.to_le_bytes();
        self.a_reg = af[0];
        // prevent writing bits 0-3
        self.flag_reg = (af[1] & 0xF0) | (self.flag_reg & 0x0F);
    }

    pub(super) fn write_bc(&mut self, val: u16) {
        let bc = val.to_le_bytes();
        self.b_reg = bc[0];
        self.c_reg = bc[1];
    }

    pub(super) fn write_de(&mut self, val: u16) {
        let de = val.to_le_bytes();
        self.d_reg = de[0];
        self.e_reg = de[1];
    }

    pub(super) fn write_hl(&mut self, val: u16) {
        let hl = val.to_le_bytes();
        self.h_reg = hl[0];
        self.l_reg = hl[1];
    }

    fn set_flag_bit(&mut self, bit: u8, high: bool) {
        if high {
            self.flag_reg |= 1 << bit;
        } else {
            self.flag_reg &= !(1 << bit);
        }
    }

    fn flag_bit(&self, bit: u8) -> bool {
        ((self.flag_reg >> bit) & 0x1) == 0x1
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
