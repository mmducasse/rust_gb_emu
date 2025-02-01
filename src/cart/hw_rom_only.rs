use crate::{
    consts::KB_32,
    mem::{array::Array, sections::Addr},
};

use super::{cart_hw::CartHw, consts::ROM_BANK_SIZE};

pub struct HwRomOnly {
    rom: Array,
}

impl HwRomOnly {
    pub fn new(rom_banks: usize) -> Self {
        Self {
            rom: Array::new(0, (rom_banks * ROM_BANK_SIZE) as u16),
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
        return self.rom.read(addr);
    }

    fn write(&mut self, addr: Addr, data: u8) {
        if !self.rom.contains_addr(addr) {
            panic!("Bad Rom-only cart write address: {}", addr);
        }
        self.rom.write(addr, data);
    }
}
