use crate::{
    cart::Cart,
    mem_map::{self, Addr},
    regs::{CpuReg16, CpuRegs},
};

pub struct Sys {
    pub cart: Cart,
    pub regs: CpuRegs,

    pub crash: bool,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            cart: Cart::new(),
            regs: CpuRegs::new(),

            crash: false,
        }
    }

    pub fn rd_mem(&mut self, addr: Addr) -> u8 {
        mem_map::read(self, addr)
    }

    pub fn wr_mem(&mut self, addr: Addr, data: u8) {
        mem_map::write(self, addr, data);
    }

    pub fn get_pc(&self) -> u16 {
        return self.regs.get_16(CpuReg16::PC);
    }

    pub fn set_pc(&mut self, data: u16) {
        self.regs.set_16(CpuReg16::PC, data);
    }
}
