use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{consts::FAIL_ON_BAD_RW, debug, sys::Sys};

use super::io_regs::IoRegs;

pub type Addr = u16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Debug)]
pub enum MemSection {
    CartRom,
    Vram,
    ExtRam,
    Wram,
    EchoRam,
    Oam,
    UnusableMemory,
    IoRegs,
    Hram,
    IeReg,
}

impl MemSection {
    /// Returns the starting address of the memory section.
    pub fn start_addr(self) -> Addr {
        match self {
            MemSection::CartRom => 0x0000,
            MemSection::Vram => 0x8000,
            MemSection::ExtRam => 0xA000,
            MemSection::Wram => 0xC000,
            MemSection::EchoRam => 0xE000,
            MemSection::Oam => 0xFE00,
            MemSection::UnusableMemory => 0xFEA0,
            MemSection::IoRegs => 0xFF00,
            MemSection::Hram => 0xFF80,
            MemSection::IeReg => 0xFFFF,
        }
    }

    pub fn size(self) -> u16 {
        if self == Self::IeReg {
            return 1;
        }

        let next = (self as usize) + 1;
        let next: Self = num::FromPrimitive::from_usize(next).unwrap();

        let section_size = next.start_addr() - self.start_addr();
        return section_size;
    }

    /// Returns the memory section that the address belongs to, as
    /// well as it's relative address within that section.
    pub fn from_abs_addr(addr: Addr) -> Self {
        for section in MemSection::iter().rev() {
            let start_addr = section.start_addr();
            if addr >= start_addr {
                return section;
            }
        }

        panic!("Unable to determine memory section of addr: {}", addr);
    }
}

pub fn read(sys: &Sys, addr: Addr) -> u8 {
    //println!("Addr = {} {:#04x}", addr, addr);
    let section = MemSection::from_abs_addr(addr);
    //println!("Rel Addr ({:?}) = {} {:#04x}", section, addr, addr);

    match section {
        MemSection::CartRom => sys.cart.rd(addr),
        MemSection::Vram => sys.vram.rd(addr),
        MemSection::ExtRam => sys.cart.rd(addr), // sys.ext_ram.rd(abs_addr),
        MemSection::Wram => sys.wram.rd(addr),
        MemSection::EchoRam => {
            if FAIL_ON_BAD_RW {
                debug::fail(sys, "Attempted to read from Echo RAM");
            } else {
                0x00
            }
        }
        MemSection::Oam => sys.oam.rd(addr),
        MemSection::UnusableMemory => {
            if FAIL_ON_BAD_RW {
                debug::fail(sys, "Attempted to read from unusable memory");
            } else {
                0x00
            }
        }
        MemSection::IoRegs => sys.io_regs.rd(addr),
        MemSection::Hram => sys.hram.rd(addr),
        MemSection::IeReg => sys.ie_reg.rd(addr),
    }
}

pub fn write(sys: &mut Sys, addr: Addr, data: u8) {
    let section = MemSection::from_abs_addr(addr);

    match section {
        MemSection::CartRom => {
            sys.cart.wr(addr, data);
        }
        MemSection::Vram => {
            sys.vram.wr(addr, data);
        }
        MemSection::ExtRam => {
            sys.cart.wr(addr, data);
        }
        MemSection::Wram => {
            sys.wram.wr(addr, data);
        }
        MemSection::EchoRam => {
            if FAIL_ON_BAD_RW {
                debug::fail(sys, "Attempted to write to Echo RAM");
            }
        }
        MemSection::Oam => {
            sys.oam.wr(addr, data);
        }
        MemSection::UnusableMemory => {
            if FAIL_ON_BAD_RW {
                debug::fail(sys, "Attempted to write to unusable memory");
            }
        }
        MemSection::IoRegs => {
            IoRegs::wr(sys, addr, data);
        }
        MemSection::Hram => {
            sys.hram.wr(addr, data);
        }
        MemSection::IeReg => {
            sys.ie_reg.wr(addr, data);
        }
    }
}

pub fn get_section_slice(sys: &Sys, section: MemSection) -> &[u8] {
    match section {
        MemSection::EchoRam | MemSection::UnusableMemory => {
            return &[];
        }
        MemSection::CartRom => sys.cart.rom(),
        MemSection::Vram => sys.vram.as_slice(),
        MemSection::ExtRam => sys.cart.ram(),
        MemSection::Wram => sys.wram.as_slice(),
        MemSection::Oam => sys.oam.as_slice(),
        MemSection::IoRegs => sys.io_regs.ram().as_slice(),
        MemSection::Hram => sys.hram.as_slice(),
        MemSection::IeReg => sys.ie_reg.as_slice(),
    }
}

pub fn print_section(sys: &Sys, section: MemSection, limit: Option<usize>) {
    let data = get_section_slice(sys, section);

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
