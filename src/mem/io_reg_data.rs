use super::io_regs::IoReg;

/// Describes special behavior for a given IO register.
pub struct IoRegData {
    reg: IoReg,
    write_mask: u8,
    reset_on_write: bool,
}

impl IoRegData {
    fn new(reg: IoReg) -> Self {
        Self {
            reg,
            write_mask: 0x00,
            reset_on_write: false,
        }
    }

    pub fn write_mask(&self) -> u8 {
        self.write_mask
    }

    pub fn reset_on_write(&self) -> bool {
        self.reset_on_write
    }

    fn with_write_mask(mut self, write_mask: u8) -> Self {
        self.write_mask = write_mask;
        self
    }

    fn with_reset_on_write(mut self) -> Self {
        self.reset_on_write = true;
        self
    }

    pub fn from_reg(reg: IoReg) -> Self {
        let r = Self::new(reg).with_write_mask(0x00);
        let rw = Self::new(reg).with_write_mask(0xFF);

        match reg {
            IoReg::P1 => rw.with_write_mask(0b1111_0000),
            IoReg::Sb => rw,
            IoReg::Sc => rw,
            IoReg::Div => rw.with_reset_on_write(),
            IoReg::Tima => rw,
            IoReg::Tma => rw,
            IoReg::Tac => rw,
            IoReg::If => rw,
            // todo: Sound regs...
            IoReg::Lcdc => rw,
            IoReg::Stat => rw.with_write_mask(0b1111_1000),
            IoReg::Scy => rw,
            IoReg::Scx => rw,
            IoReg::Ly => r,
            IoReg::Lyc => rw,
            IoReg::Dma => rw,
            IoReg::Bgp => rw,
            IoReg::Obp0 => rw,
            IoReg::Obp1 => rw,
            IoReg::Wy => rw,
            IoReg::Wx => rw,
            // todo: CGB regs...
            IoReg::Ie => rw,
        }
    }
}
