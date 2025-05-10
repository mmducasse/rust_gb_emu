use crate::{mem::Addr, util::math::bits8};

use super::{
    cart_hw::CartHw,
    consts::{RAM_BANK_SIZE, ROM_BANK_SIZE},
};

/// MBC3 cartridge hardware. Features 2MB ROM and/or 64KB RAM, and Timer.
pub struct HwMbc3 {
    rom: Vec<u8>,
    rom_bank_sel: u8,

    ram: Vec<u8>,
    ram_timer_enable: bool,
    ram_rtc_register: u8,
    ram_bank_rtc_reg_sel: u8,
    //latch_clock_data: u8,
    //day_ctr: u8,
}

impl HwMbc3 {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Self {
            rom: vec![0; rom_banks * ROM_BANK_SIZE],

            ram: vec![0; ram_banks * RAM_BANK_SIZE],
            rom_bank_sel: 0,
            ram_timer_enable: false,
            ram_rtc_register: 0,
            ram_bank_rtc_reg_sel: 0,
            //latch_clock_data: 0,
            //day_ctr: 0,
        }
    }

    pub fn rom_bank_sel(&self) -> u8 {
        self.rom_bank_sel
    }

    pub fn ram_bank_sel(&self) -> u8 {
        self.ram_rtc_register
    }
}

impl CartHw for HwMbc3 {
    fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram
    }

    fn read(&self, addr: Addr) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                // ROM Bank 01-7F
                let rel_addr = addr - 0x4000;
                let bank_sel = self.rom_bank_sel() as usize;
                let bank_offs = bank_sel * ROM_BANK_SIZE;
                let addr = bank_offs + (rel_addr as usize);
                if addr >= self.rom.len() {
                    return 0;
                }
                self.rom[addr]
            }
            0xA000..=0xBFFF => {
                // RAM Bank 01-7F
                let rel_addr = addr - 0x4000;
                let bank_sel = self.ram_bank_sel() as usize;
                let bank_offs = bank_sel * RAM_BANK_SIZE;
                let addr = bank_offs + (rel_addr as usize);
                if addr >= self.rom.len() {
                    return 0;
                }
                self.ram[addr]
            }
            _ => {
                panic!("Invalid MBC3 read address");
            }
        }
    }

    fn write(&mut self, addr: Addr, data: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_timer_enable = bits8(&data, 3, 0) == 0xA;
            }
            0x2000..=0x3FFF => {
                self.rom_bank_sel = bits8(&data, 6, 0);
            }
            0x4000..=0x5FFF => {
                self.ram_bank_rtc_reg_sel = data;
            }
            0x6000..=0x7FFF => {
                // todo: latch clock registers
            }
            0xA000..=0xBFFF => {
                // RAM Bank 00-03
                let rel_addr = addr - 0xA000;
                let bank_offs = (self.ram_bank_sel() as usize) * 0x2000;
                let addr = bank_offs + (rel_addr as usize);

                if addr >= self.ram.len() {
                    return;
                }

                self.ram[addr] = data;
            }
            _ => {
                panic!("Invalid MBC3 write address");
            }
        }
    }
}
