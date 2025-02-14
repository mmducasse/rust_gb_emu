use super::io_regs::IoReg;

/// Describes special behavior for a given IO register.
pub struct IoRegData {
    reg: IoReg,
    read_mask: u8,
    write_mask: u8,
}

impl IoRegData {
    fn new(reg: IoReg) -> Self {
        Self {
            reg,
            read_mask: 0xFF,
            write_mask: 0xFF,
        }
    }

    pub fn read_mask(&self) -> u8 {
        self.read_mask
    }

    pub fn write_mask(&self) -> u8 {
        self.write_mask
    }

    fn with_read_mask(mut self, read_mask: u8) -> Self {
        self.read_mask = read_mask;
        self
    }

    fn with_write_mask(mut self, write_mask: u8) -> Self {
        self.write_mask = write_mask;
        self
    }

    pub fn from_reg(reg: IoReg) -> Self {
        let data = Self::new(reg);

        match reg {
            IoReg::P1 => data.with_write_mask(0b1111_0000),
            IoReg::Sb => data,
            IoReg::Sc => data,
            IoReg::Div => data,
            IoReg::Tima => data,
            IoReg::Tma => data,
            IoReg::Tac => data,
            IoReg::If => data
                .with_read_mask(0b0001_1111)
                .with_write_mask(0b0001_1111),
            // todo: Sound regs...
            IoReg::Lcdc => data,
            IoReg::Stat => data.with_write_mask(0b1111_1000),
            IoReg::Scy => data,
            IoReg::Scx => data,
            IoReg::Ly => data.with_write_mask(0x00),
            IoReg::Lyc => data,
            IoReg::Dma => data,
            IoReg::Bgp => data,
            IoReg::Obp0 => data,
            IoReg::Obp1 => data,
            IoReg::Wy => data,
            IoReg::Wx => data,
            // todo: CGB regs...
            IoReg::Ie => data,
        }
    }
}
