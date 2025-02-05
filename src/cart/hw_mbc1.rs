use num::FromPrimitive;

use crate::{
    consts::{KB_32, MB_2},
    debug,
    mem::sections::Addr,
    util::math::{bit8, bits8, set_bits8},
};

use super::{
    cart_hw::CartHw,
    consts::{RAM_BANK_SIZE, ROM_BANK_SIZE},
};

#[derive(Clone, Copy, PartialEq, Eq, FromPrimitive)]
enum Mode {
    RomBanking,
    RamBanking,
}

const ROM_MAX_SIZE: usize = MB_2;
const RAM_MAX_SIZE: usize = KB_32;

pub struct HwMbc1 {
    rom: Vec<u8>,

    ram: Vec<u8>,
    ram_enable: bool,

    bank_sel_lower_5: u8,
    bank_sel_upper_2: u8,

    mode_sel: Mode,
}

impl HwMbc1 {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Self {
            rom: vec![0; rom_banks * ROM_BANK_SIZE],

            ram: vec![0; ram_banks * RAM_BANK_SIZE],
            ram_enable: false,

            bank_sel_lower_5: 0,
            bank_sel_upper_2: 0,

            mode_sel: Mode::RamBanking,
        }
    }

    pub fn rom_bank_sel(&self) -> u8 {
        let mut lower = bits8(&self.bank_sel_lower_5, 4, 0);
        if (lower & 0x1F) == 0 {
            lower += 1;
        }
        let upper = if self.mode_sel == Mode::RomBanking {
            bits8(&self.bank_sel_upper_2, 1, 0)
        } else {
            0
        };

        let bank = (upper << 5) | lower;
        return bank;
    }

    pub fn ram_bank_sel(&self) -> u8 {
        return if self.mode_sel == Mode::RamBanking {
            bits8(&self.bank_sel_upper_2, 1, 0)
        } else {
            0
        };
    }
}

impl CartHw for HwMbc1 {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram
    }

    // todo cleanup
    fn read(&self, addr: Addr) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                // ROM Bank 00
                self.rom[addr as usize]
            }
            0x4000..=0x7FFF => {
                // ROM Bank 01-7F
                let rel_addr = addr - 0x4000;
                let bank_sel = self.rom_bank_sel() as usize;
                let bank_offs = bank_sel * ROM_BANK_SIZE;
                let addr = bank_offs + (rel_addr as usize);
                if addr >= self.rom.len() {
                    // debug::fail(format!(
                    //     "Attempted to read MCB1 ROM address {:0>4X} (len = {:0>8X})",
                    //     addr,
                    //     self.rom.len()
                    // ));
                    return 0;
                }
                self.rom[addr]
            }
            0xA000..=0xBFFF => {
                // RAM Bank 00-03
                let rel_addr = addr - 0xA000;
                let bank_offs = (self.ram_bank_sel() as usize) * RAM_BANK_SIZE;
                let addr = bank_offs + (rel_addr as usize);

                if addr >= self.ram.len() {
                    // debug::fail(format!(
                    //     "Attempted to read MCB1 RAM address {:0>4X} (len = {:0>8X})",
                    //     addr,
                    //     self.ram.len()
                    // ));
                    return 0;
                }

                self.ram[addr]
            }
            _ => {
                panic!("Invalid MBC1 read address");
            }
        }
    }

    // todo cleanup
    fn write(&mut self, addr: Addr, data: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enable = bits8(&data, 3, 0) == 0xA;
            }
            0x2000..=0x3FFF => {
                self.bank_sel_lower_5 = data;
            }
            0x4000..=0x5FFF => {
                self.bank_sel_upper_2 = data;
            }
            0x6000..=0x7FFF => {
                self.mode_sel = Mode::from_u8(bit8(&data, 0))
                    .expect(&format!("Invalid MBC1 banking mode value: {}.", data));
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    // RAM Bank 00-03
                    let rel_addr = addr - 0xA000;
                    let bank_offs = (self.ram_bank_sel() as usize) * 0x2000;
                    let addr = bank_offs + (rel_addr as usize);

                    if addr >= self.ram.len() {
                        // debug::fail(format!(
                        //     "Attempted to write MCB1 RAM address {:0>4X} (len = {:0>8X})",
                        //     addr,
                        //     self.ram.len()
                        // ));
                        return;
                    }

                    self.ram[addr] = data;
                }
            }
            _ => {
                panic!("Invalid MBC1 write address");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mbc1_rw_rom() {
        let mut hw = HwMbc1::new(0xFF, 0x04);

        hw.write(0x6000, Mode::RomBanking as u8);

        for upper in 0..=3 {
            for mut lower in 0..=0x1F {
                hw.write(0x2000, lower);
                hw.write(0x4000, upper);

                if (lower & 0x1F) == 0 {
                    lower += 1;
                }
                let bank = (upper << 5) | lower;
                let addr = bank as usize * ROM_BANK_SIZE;
                let write_value = bank;
                hw.rom_mut()[addr] = write_value;
                let read_value = hw.read(0x4000);

                //assert_eq!(write_value, read_value);
                println!("Read bank {:0>4X}: {:0>4X}", bank, read_value);
            }
        }
    }

    #[test]
    fn test_mbc1_rw_ram() {
        let mut hw = HwMbc1::new(0xFF, 0x04);

        hw.write(0x6000, Mode::RamBanking as u8);

        for upper in 0..=3 {
            for lower in 0..=0x1F {
                hw.write(0x2000, lower);
                hw.write(0x4000, upper);

                let bank = upper;
                let addr = bank as usize * RAM_BANK_SIZE;
                let write_value = bank;
                hw.ram_mut()[addr] = write_value;
                let read_value = hw.read(0xA000);

                //assert_eq!(write_value, read_value);
                println!("Read bank {:0>4X}: {:0>4X}", bank, read_value);
            }
        }
    }
}
