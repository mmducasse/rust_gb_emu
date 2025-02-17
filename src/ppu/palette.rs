use crate::{mem::io_regs::IoReg, sys::Sys};

/// Maps color IDs to color values.
pub struct Palette {
    pub val0: u8,
    pub val1: u8,
    pub val2: u8,
    pub val3: u8,
}

impl Palette {
    /// Loads a `Palette` from an IO register.
    pub fn from_reg(sys: &Sys, reg: IoReg) -> Self {
        let data = sys.mem.io_regs.get(reg);
        return Self::new(data);
    }

    pub const fn new(data: u8) -> Self {
        Self {
            val0: (data >> 0) & 0b11,
            val1: (data >> 2) & 0b11,
            val2: (data >> 4) & 0b11,
            val3: (data >> 6) & 0b11,
        }
    }

    pub const fn default() -> Self {
        Self {
            val0: 0,
            val1: 1,
            val2: 2,
            val3: 3,
        }
    }

    pub fn map(&self, color_id: u8) -> u8 {
        match color_id {
            0b00 => self.val0,
            0b01 => self.val1,
            0b10 => self.val2,
            0b11 => self.val3,
            _ => unreachable!(),
        }
    }
}
