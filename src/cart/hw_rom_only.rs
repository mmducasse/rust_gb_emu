use crate::{consts::KB_32, mem::map::Addr};

use super::cart_hw::CartHw;

pub struct HwRomOnly {
    rom: Vec<u8>,
}

impl HwRomOnly {
    pub fn new() -> Self {
        Self {
            rom: vec![0; KB_32],
        }
    }
}

impl CartHw for HwRomOnly {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    fn rd(&self, addr: Addr) -> u8 {
        self.rom[addr as usize]
    }

    fn wr(&mut self, addr: Addr, data: u8) {
        self.rom[addr as usize] = data;
    }
}
