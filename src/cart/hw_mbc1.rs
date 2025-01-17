use num::FromPrimitive;

use crate::{
    consts::{KB_32, MB_2},
    mem::map::Addr,
    util::math::{bit8, bits8, set_bits8},
};

use super::cart_hw::CartHw;

#[derive(Clone, Copy, FromPrimitive)]
enum Mode {
    RomBanking,
    RamBanking,
}

const ROM_MAX_SIZE: usize = MB_2;
const RAM_MAX_SIZE: usize = KB_32;

pub struct HwMbc1 {
    rom: Vec<u8>,
    rom_bank_sel: u8,

    ram: Vec<u8>,
    ram_bank_sel: u8,
    ram_enable: bool,

    mode_sel: Mode,
}

impl HwMbc1 {
    pub fn new() -> Self {
        Self {
            rom: vec![0; ROM_MAX_SIZE],
            rom_bank_sel: 0,

            ram: vec![0; RAM_MAX_SIZE],
            ram_bank_sel: 0,
            ram_enable: false,

            mode_sel: Mode::RomBanking,
        }
    }
}

impl CartHw for HwMbc1 {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    // todo cleanup
    fn rd(&self, addr: Addr) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                // ROM Bank 00
                self.rom[addr as usize]
            }
            0x4000..=0x7FFF => {
                // ROM Bank 01-7F
                let rel_addr = addr - 0x4000;
                let bank_offs = (self.rom_bank_sel as usize) * 0x4000;
                let addr = bank_offs + (rel_addr as usize);
                self.rom[addr]
            }
            0xA000..=0xBFFF => {
                // RAM Bank 00-03
                let rel_addr = addr - 0xA000;
                let bank_offs = (self.ram_bank_sel as usize) * 0x2000;
                let addr = bank_offs + (rel_addr as usize);
                self.ram[addr]
            }
            _ => {
                panic!("Invalid MBC1 read address");
            }
        }
    }

    // todo cleanup
    fn wr(&mut self, addr: Addr, data: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enable = bits8(&data, 3, 0) == 0xA;
            }
            0x2000..=0x3FFF => {
                let mut bank_lower = bits8(&data, 4, 0);
                if bank_lower % 0x20 == 0 {
                    bank_lower += 1;
                }
                set_bits8(&mut self.rom_bank_sel, 4, 0, bank_lower);
            }
            0x4000..=0x5FFF => {
                let bits = bits8(&data, 1, 0);
                match self.mode_sel {
                    Mode::RomBanking => {
                        set_bits8(&mut self.rom_bank_sel, 6, 5, bits);
                    }
                    Mode::RamBanking => {
                        self.ram_bank_sel = bits;
                    }
                }
            }
            0x6000..=0x7FFF => {
                self.mode_sel = Mode::from_u8(bit8(&data, 0))
                    .expect(&format!("Invalid MBC1 banking mode value: {}.", data));
            }
            _ => {
                panic!("Invalid MBC1 write address");
            }
        }
    }
}
