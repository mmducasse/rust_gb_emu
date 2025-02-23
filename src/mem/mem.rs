use crate::{cart::cart::Cart, consts::FAIL_ON_BAD_RW, debug};

use super::{array::Array, io_regs::IoRegs, sections::MemSection, Addr};

pub struct Mem {
    pub cart: Cart,
    pub wram: Array,
    pub vram: Array,
    pub oam: Array,
    pub io_regs: IoRegs,
    pub hram: Array,
}

impl Mem {
    pub fn new(cart: Cart) -> Self {
        Self {
            cart,
            wram: MemSection::into_array(MemSection::Wram),
            vram: MemSection::into_array(MemSection::Vram),
            oam: MemSection::into_array(MemSection::Oam),
            io_regs: IoRegs::new(),
            hram: MemSection::into_array(MemSection::Hram),
        }
    }

    pub fn read(&self, addr: Addr) -> u8 {
        //println!("Addr = {} {:#04x}", addr, addr);
        let section = MemSection::from_abs_addr(addr);
        //println!("Rel Addr ({:?}) = {} {:#04x}", section, addr, addr);

        match section {
            MemSection::CartRom => self.cart.read(addr),
            MemSection::Vram => self.vram.read(addr),
            MemSection::ExtRam => self.cart.read(addr), // sys.ext_ram.rd(abs_addr),
            MemSection::Wram => self.wram.read(addr),
            MemSection::EchoRam => {
                if FAIL_ON_BAD_RW {
                    debug::fail("Attempted to read from Echo RAM");
                }
                0x00
            }
            MemSection::Oam => self.oam.read(addr),
            MemSection::UnusableMemory => {
                if FAIL_ON_BAD_RW {
                    debug::fail("Attempted to read from unusable memory");
                }
                0x00
            }
            MemSection::IoRegs => self.io_regs.user_read(addr),
            MemSection::Hram => self.hram.read(addr),
            MemSection::IeReg => self.io_regs.user_read(addr),
        }
    }

    pub fn write(&mut self, addr: Addr, data: u8) {
        let section = MemSection::from_abs_addr(addr);

        match section {
            MemSection::CartRom => {
                self.cart.write(addr, data);
            }
            MemSection::Vram => {
                self.vram.write(addr, data);
            }
            MemSection::ExtRam => {
                self.cart.write(addr, data);
            }
            MemSection::Wram => {
                self.wram.write(addr, data);
            }
            MemSection::EchoRam => {
                if FAIL_ON_BAD_RW {
                    //debug::fail(sys, "Attempted to write to Echo RAM");
                }
            }
            MemSection::Oam => {
                self.oam.write(addr, data);
            }
            MemSection::UnusableMemory => {
                if FAIL_ON_BAD_RW {
                    //debug::fail(sys, "Attempted to write to unusable memory");
                }
            }
            MemSection::IoRegs => {
                self.io_regs.user_write(addr, data);
            }
            MemSection::Hram => {
                self.hram.write(addr, data);
            }
            MemSection::IeReg => {
                self.io_regs.user_write(addr, data);
            }
        }
    }
}
