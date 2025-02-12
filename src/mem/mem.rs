use crate::{cart::cart::Cart, consts::FAIL_ON_BAD_RW, cpu::regs::CpuRegs, debug};

use super::{
    array::Array,
    io_regs::IoRegs,
    sections::{Addr, MemSection},
};

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
            wram: Array::from_mem_section(MemSection::Wram),
            vram: Array::from_mem_section(MemSection::Vram),
            oam: Array::from_mem_section(MemSection::Oam),
            io_regs: IoRegs::new(),
            hram: Array::from_mem_section(MemSection::Hram),
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

    pub fn get_section_slice(&self, section: MemSection) -> &[u8] {
        match section {
            MemSection::EchoRam | MemSection::UnusableMemory => {
                return &[];
            }
            MemSection::CartRom => self.cart.rom(),
            MemSection::Vram => self.vram.as_slice(),
            MemSection::ExtRam => self.cart.ram(),
            MemSection::Wram => self.wram.as_slice(),
            MemSection::Oam => self.oam.as_slice(),
            MemSection::IoRegs => self.io_regs.ram().as_slice(),
            MemSection::Hram => self.hram.as_slice(),
            MemSection::IeReg => self.io_regs.ie().as_slice(),
        }
    }

    pub fn print_section(&self, section: MemSection, limit: Option<usize>) {
        let data = self.get_section_slice(section);

        println!("Mem section: {:?}", section);
        let start = section.start_addr();
        for (idx, data) in data.iter().enumerate() {
            let addr = start + (idx as u16);
            println!("  [{:0>4X}] {:0>2X}", addr, *data);

            if let Some(limit) = limit {
                if idx >= limit {
                    println!("  ...");
                    break;
                }
            }
        }
        println!();
    }
}
