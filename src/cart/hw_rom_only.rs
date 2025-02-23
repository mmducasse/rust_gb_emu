use crate::mem::Addr;

use super::{cart_hw::CartHw, consts::ROM_BANK_SIZE};

/// Cartridge hardware with only ROM.
pub struct HwRomOnly {
    rom: Vec<u8>,
}

impl HwRomOnly {
    pub fn new(rom_banks: usize) -> Self {
        Self {
            rom: vec![0; rom_banks * ROM_BANK_SIZE],
        }
    }
}

impl CartHw for HwRomOnly {
    fn rom_mut(&mut self) -> &mut [u8] {
        self.rom.as_mut_slice()
    }

    fn ram(&self) -> &[u8] {
        &[]
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        &mut []
    }

    fn read(&self, addr: Addr) -> u8 {
        let addr = addr as usize;
        return *self.rom.get(addr).unwrap_or(&0);
    }

    fn write(&mut self, _: Addr, _: u8) {
        // Does nothing.
    }
}
