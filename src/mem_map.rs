use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::sys::Sys;

pub type Addr = u16;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Debug)]
pub enum MemSection {
    CartRom,
    Vram,
    ExternalRam,
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
            MemSection::ExternalRam => 0xA000,
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
    pub fn from_addr(addr: Addr) -> (Self, Addr) {
        for section in MemSection::iter().rev() {
            let start_addr = section.start_addr();
            if addr >= start_addr {
                let rel_addr = addr - start_addr;
                return (section, rel_addr);
            }
        }

        panic!("Unable to determine memory section of addr: {}", addr);
    }
}

pub fn read(sys: &mut Sys, addr: Addr) -> u8 {
    let (section, addr) = MemSection::from_addr(addr);

    match section {
        MemSection::CartRom => {
            todo!("Read from cart ROM");
        }
        MemSection::Vram => {
            todo!("Read from VRAM");
        }
        MemSection::ExternalRam => {
            todo!("Read from Ext RAM");
        }
        MemSection::Wram => {
            todo!("Read from WRAM");
        }
        MemSection::EchoRam => {
            panic!("Attempted to read from Echo RAM");
        }
        MemSection::Oam => {
            todo!("Read from OAM");
        }
        MemSection::UnusableMemory => {
            panic!("Attempted to read from unusable memory");
        }
        MemSection::IoRegs => {
            todo!("Read from I/O regs");
        }
        MemSection::Hram => {
            todo!("Read from HRAM");
        }
        MemSection::IeReg => {
            todo!("Read from IE reg");
        }
    }
}

pub fn write(sys: &mut Sys, addr: Addr, data: u8) {
    let (section, addr) = MemSection::from_addr(addr);

    match section {
        MemSection::CartRom => {
            todo!("Write to cart ROM");
        }
        MemSection::Vram => {
            todo!("Write to VRAM");
        }
        MemSection::ExternalRam => {
            todo!("Write to Ext RAM");
        }
        MemSection::Wram => {
            todo!("Write to WRAM");
        }
        MemSection::EchoRam => {
            panic!("Attempted to write to Echo RAM");
        }
        MemSection::Oam => {
            todo!("Write to OAM");
        }
        MemSection::UnusableMemory => {
            panic!("Attempted to write to unusable memory");
        }
        MemSection::IoRegs => {
            todo!("Write to  I/O regs");
        }
        MemSection::Hram => {
            todo!("Write to  HRAM");
        }
        MemSection::IeReg => {
            todo!("Write to  IE reg");
        }
    }
}
