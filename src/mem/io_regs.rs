use super::{
    map::{Addr, MemSection},
    mem::Mem,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IoReg {
    P1 = 0xFF00,
    Sb = 0xFF01,
    Sc = 0xFF02,
    Div = 0xFF04,
    Tima = 0xFF05,
    Tma = 0xFF06,
    Tac = 0xFF07,
    If = 0xFF0F,

    // NR10..NR52
    Lcdc = 0xFF40,
    Stat = 0xFF41,
    Scy = 0xFF42,
    Scx = 0xFF43,
    Ly = 0xFF44,
    Lyc = 0xFF45,
    Dma = 0xFF46,
    Bgp = 0xFF47,
    Obp0 = 0xFF48,
    Obp1 = 0xFF49,
    Wy = 0xFF4A,
    Wx = 0xFF4B,
    Key1 = 0xFF4D,
    Vbk = 0xFF4F,
    Hdma1 = 0xFF51,
    Hdma2 = 0xFF52,
    Hdma3 = 0xFF53,
    Hdma4 = 0xFF54,
    Hdma5 = 0xFF55,
    Rp = 0xFF56,
    Bcps = 0xFF68,
    Bcpd = 0xFF69,
    Ocps = 0xFF6A,
    Ocpd = 0xFF6B,
    Svbk = 0xFF70,
    Ie = 0xFFFF,
}

impl Into<Addr> for IoReg {
    fn into(self) -> Addr {
        return self as Addr;
    }
}

pub struct IoRegs {
    mem: Mem,
}

impl IoRegs {
    pub fn new() -> Self {
        Self {
            mem: Mem::from_mem_section(MemSection::IoRegs),
        }
    }

    pub fn ram(&self) -> &Mem {
        &self.mem
    }

    pub fn rd(&self, addr: Addr) -> u8 {
        return self.mem.rd(addr);
    }

    pub fn wr(&mut self, addr: Addr, data: u8) {
        let div: Addr = IoReg::Div.into();
        if addr == div {
            self.mem.wr(IoReg::Div, 0x00);
        } else {
            self.mem.wr(addr, data);
        }
    }
}
