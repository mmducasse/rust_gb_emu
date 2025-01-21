use crate::{cart::cart::Cart, cpu::regs::CpuRegs};

use super::{array::Array, io_regs::IoRegs, map::MemSection};

pub struct Mem {
    pub cart: Cart,
    pub wram: Array,
    pub vram: Array,
    pub oam: Array,
    pub io_regs: IoRegs,
    pub hram: Array,
    pub ie_reg: Array,
}

impl Mem {
    pub fn new() -> Self {
        Self {
            cart: Cart::new(),
            wram: Array::from_mem_section(MemSection::Wram),
            vram: Array::from_mem_section(MemSection::Vram),
            oam: Array::from_mem_section(MemSection::Oam),
            io_regs: IoRegs::new(),
            hram: Array::from_mem_section(MemSection::Hram),
            ie_reg: Array::from_mem_section(MemSection::IeReg),
        }
    }
}
