use std::collections::HashMap;

use io_reg_data::IoRegData;
use num::FromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{debug, util::math::set_bits8_masked};

use super::{array::Array, sections::MemSection, Addr};

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
    pub fn as_addr(self) -> Addr {
        self.into()
    }
}

impl From<IoReg> for Addr {
    fn from(val: IoReg) -> Self {
        val as Addr
    }
}

pub struct IoRegs {
    mem: Array,
    ie: Array, // IE reg is not part of contiguous IO regs memory.
    reg_datas: HashMap<IoReg, IoRegData>,

    pub dma_requested: bool,
}

impl IoRegs {
    pub fn new() -> Self {
        let mut reg_datas = HashMap::new();
        for reg in IoReg::iter() {
            let reg_data = IoRegData::from_reg(reg);
            reg_datas.insert(reg, reg_data);
        }

        Self {
            mem: MemSection::into_array(MemSection::IoRegs),
            ie: MemSection::into_array(MemSection::IeReg),
            reg_datas,

            dma_requested: false,
        }
    }

    /// Reads from the readable bits in the IO register.
    pub fn user_read(&self, addr: Addr) -> u8 {
        let mut data = if addr == IoReg::Ie.as_addr() {
            self.ie.read(addr)
        } else {
            self.mem.read(addr)
        };

        if let Some(reg) = IoReg::from_u16(addr) {
            debug::record_io_reg_usage(reg, false, 0x00);
            let Some(reg_data) = self.reg_datas.get(&reg) else {
                unreachable!();
            };

            data &= reg_data.read_mask();
        }

        data
    }

    /// Reads the entire IO register.
    pub fn get(&self, reg: IoReg) -> u8 {
        if reg == IoReg::Ie {
            self.ie.read(reg)
        } else {
            self.mem.read(reg)
        }
    }

    /// Writes to the writeable bits in the IO register.
    pub fn user_write(&mut self, addr: Addr, value: u8) {
        if addr == IoReg::Ie.as_addr() {
            debug::record_io_reg_usage(IoReg::Ie, true, value);
            self.ie.write(addr, value);
        } else if let Some(reg) = IoReg::from_u16(addr) {
            debug::record_io_reg_usage(reg, true, value);
            let Some(reg_data) = self.reg_datas.get(&reg) else {
                unreachable!();
            };

            if reg == IoReg::Sc {
                let serial_data = self.get(IoReg::Sb);
                debug::push_serial_char(serial_data as char);
            } else if reg == IoReg::Dma {
                self.dma_requested = true;
            }

            if reg == IoReg::Div {
                self.mem.write(addr, 0x00);
            } else {
                let data = self.mem.mut_(addr);
                let mask = reg_data.write_mask();
                set_bits8_masked(data, mask, value);
            }
        } else {
            self.mem.write(addr, value);
        }
    }

    /// Sets the entire IO register.
    pub fn set(&mut self, reg: IoReg, data: u8) {
        if reg == IoReg::Ie {
            self.ie.write(reg, data)
        } else {
            self.mem.write(reg, data)
        }
    }

    /// Gets a mutable reference to the IO register.
    pub fn mut_(&mut self, reg: IoReg, mut f: impl FnMut(&mut u8)) -> u8 {
        let data = if reg == IoReg::Ie {
            self.ie.mut_(reg)
        } else {
            self.mem.mut_(reg)
        };

        f(data);

        *data
    }
}

mod io_reg_data {
    use super::IoReg;

    /// Describes special behavior for a given IO register.
    pub struct IoRegData {
        read_mask: u8,
        write_mask: u8,
    }

    impl IoRegData {
        pub fn read_mask(&self) -> u8 {
            self.read_mask
        }

        pub fn write_mask(&self) -> u8 {
            self.write_mask
        }

        pub fn from_reg(reg: IoReg) -> Self {
            let read_mask = match reg {
                IoReg::If => 0b0001_1111,
                _ => 0xFF,
            };

            let write_mask = match reg {
                IoReg::If => 0b0001_1111,
                IoReg::Stat => 0b1111_1000,
                IoReg::Ly => 0b0000_0000,
                _ => 0xFF,
            };

            Self {
                read_mask,
                write_mask,
            }
        }
    }
}
