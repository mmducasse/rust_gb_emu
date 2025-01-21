use crate::{
    consts::KB_32,
    mem::{array::Array, map::Addr},
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

    fn rd(&self, addr: Addr) -> u8 {
        if self.rom.contains_addr(addr) {
            return self.rom.rd(addr);
        }
        return 0x00;
    }

    fn wr(&mut self, addr: Addr, data: u8) {
        if self.rom.contains_addr(addr) {
            self.rom.wr(addr, data);
        }
    }
}
