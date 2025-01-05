use crate::util::math::{bit8, set_bit8};

use super::{
    map::{Addr, MemSection},
    ram::Ram,
};

pub const DIV_ADDR: Addr = 0xFF04;
pub const TIMA_ADDR: Addr = 0xFF05;
pub const TMA_ADDR: Addr = 0xFF06;
pub const TAC_ADDR: Addr = 0xFF07;

pub const IF_ADDR: Addr = 0xFF0F;
pub const IE_ADDR: Addr = 0xFFFF;

pub struct IoRegs {
    ram: Ram,
}

impl IoRegs {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(MemSection::IoRegs.size()),
        }
    }

    pub fn rd(&self, addr: Addr) -> u8 {
        return self.ram.rd(addr);
    }

    pub fn wr(&mut self, addr: Addr, data: u8) {
        if addr == DIV_ADDR {
            self.ram.wr(DIV_ADDR, 0x00);
        } else {
            self.ram.wr(addr, data);
        }
    }
}
