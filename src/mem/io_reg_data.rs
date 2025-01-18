use super::io_regs::IoReg;

/// Describes special behavior for a given IO register.
pub struct IoRegData {
    reg: IoReg,
    rd_mask: u8,
    wr_mask: u8,

    reset_on_write: bool,
}

impl IoRegData {
    fn new(reg: IoReg, rd_mask: u8, wr_mask: u8) -> Self {
        Self {
            reg,
            rd_mask,
            wr_mask,
            reset_on_write: false,
        }
    }

    pub fn read_mask(&self) -> u8 {
        self.rd_mask
    }

    pub fn write_mask(&self) -> u8 {
        self.wr_mask
    }

    pub fn reset_on_write(&self) -> bool {
        self.reset_on_write
    }

    fn with_reset_on_write(mut self) -> Self {
        self.reset_on_write = true;
        self
    }

    pub fn from_reg(reg: IoReg) -> Self {
        let r = Self::new(reg, 0xFF, 0x00);
        let w = Self::new(reg, 0x00, 0xFF);
        let rw = Self::new(reg, 0xFF, 0xFF);
        let mixed = |rd_mask: u8, wr_mask: u8| Self::new(reg, rd_mask, wr_mask);

        match reg {
            IoReg::P1 => mixed(0xFF, 0b1111_0000),
            IoReg::Sb => rw,
            IoReg::Sc => rw,
            IoReg::Div => rw.with_reset_on_write(),
            IoReg::Tima => rw,
            IoReg::Tma => rw,
            IoReg::Tac => rw,
            IoReg::If => rw,
            // todo: Sound regs...
            IoReg::Lcdc => rw,
            IoReg::Stat => mixed(0b1111_1111, 0b1111_1000),
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
