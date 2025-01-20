use std::collections::HashMap;

use num::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::util::math::{set_bits8, set_bits8_masked};

use super::{
    io_reg_data::IoRegData,
    map::{Addr, MemSection},
    mem::Mem,
};

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, FromPrimitive, EnumIter)]
pub enum IoReg {
    P1 = 0xFF00,
    Sb = 0xFF01,
    Sc = 0xFF02,
    Div = 0xFF04,
    Tima = 0xFF05,
    Tma = 0xFF06,
    Tac = 0xFF07,
    If = 0xFF0F,

    // NR10..NR52
    Lcdc = 0xFF40,
    Stat = 0xFF41,
    Scy = 0xFF42,
    Scx = 0xFF43,
    Ly = 0xFF44,
    Lyc = 0xFF45,
    Dma = 0xFF46,
    Bgp = 0xFF47,
    Obp0 = 0xFF48,
    Obp1 = 0xFF49,
    Wy = 0xFF4A,
    Wx = 0xFF4B,
    // Key1 = 0xFF4D,
    // Vbk = 0xFF4F,
    // Hdma1 = 0xFF51,
    // Hdma2 = 0xFF52,
    // Hdma3 = 0xFF53,
    // Hdma4 = 0xFF54,
    // Hdma5 = 0xFF55,
    // Rp = 0xFF56,
    // Bcps = 0xFF68,
    // Bcpd = 0xFF69,
    // Ocps = 0xFF6A,
    // Ocpd = 0xFF6B,
    // Svbk = 0xFF70,
    Ie = 0xFFFF,
}

impl IoReg {
    pub fn as_u16(self) -> Addr {
        self.into()
    }
}

impl Into<Addr> for IoReg {
    fn into(self) -> Addr {
        return self as Addr;
    }
}

pub struct IoRegs {
    mem: Mem,
    reg_datas: HashMap<IoReg, IoRegData>,
}

impl IoRegs {
    pub fn new() -> Self {
        let mut reg_datas = HashMap::new();
        for reg in IoReg::iter() {
            let reg_data = IoRegData::from_reg(reg);
            reg_datas.insert(reg, reg_data);
        }

        return Self {
            mem: Mem::from_mem_section(MemSection::IoRegs),
            reg_datas,
        };
    }

    pub fn ram(&self) -> &Mem {
        &self.mem
    }

    pub fn rd(&self, addr: Addr) -> u8 {
        if let Some(reg) = IoReg::from_u16(addr) {
            //println!("Read IO reg: {:?}", reg);
            let Some(reg_data) = self.reg_datas.get(&reg) else {
                unreachable!();
            };

            let data = self.mem.rd(addr);
            return data & reg_data.read_mask();
        } else {
            return self.mem.rd(addr);
        }
    }

    pub fn wr(&mut self, addr: Addr, value: u8) {
        if let Some(reg) = IoReg::from_u16(addr) {
            //println!("Write IO reg: {:?}: {:0>4X}", reg, value);
            let Some(reg_data) = self.reg_datas.get(&reg) else {
                unreachable!();
            };

            if reg_data.reset_on_write() {
                self.mem.wr(addr, 0x00);
            } else {
                let data = self.mem.mut_(addr);
                let mask = reg_data.write_mask();
                set_bits8_masked(data, mask, value);
            }
        } else {
            self.mem.wr(addr, value);
        }
    }

    pub fn mut_(&mut self, reg: IoReg) -> &mut u8 {
        return self.mem.mut_(reg);
    }
}
