use crate::{
    consts::KB_32,
    mem::{array::Array, sections::Addr},
};

use super::cart_hw::CartHw;

pub struct HwRomOnly {
    rom: Array,
}

impl HwRomOnly {
    pub fn new() -> Self {
        Self {
            rom: Array::new(0, KB_32 as u16),
        }
    }
}

impl CartHw for HwRomOnly {
    fn rom(&self) -> &[u8] {
        self.rom.as_slice()
    }

    fn rom_mut(&mut self) -> &mut [u8] {
        self.rom.as_mut_slice()
    }

    fn ram(&self) -> &[u8] {
        &[]
    }

    fn read(&self, addr: Addr) -> u8 {
        if !self.rom.contains_addr(addr) {
            panic!("Bad Rom-only cart read address: {}", addr);
        }
        return self.rom.rd(addr);
    }

    fn write(&mut self, addr: Addr, data: u8) {
        if !self.rom.contains_addr(addr) {
            panic!("Bad Rom-only cart write address: {}", addr);
        }
        self.rom.wr(addr, data);
    }
}
