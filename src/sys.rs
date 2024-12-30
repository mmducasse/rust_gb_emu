use crate::{
    cart::Cart,
    mem_map::{self, Addr},
    regs::CpuRegs,
};

pub struct Sys {
    pub cart: Cart,
    pub regs: CpuRegs,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            cart: Cart::new(),
            regs: CpuRegs::new(),
        }
    }

    pub fn rd_mem(&mut self, addr: Addr) -> u8 {
        mem_map::read(self, addr)
    }

    pub fn wr_mem(&mut self, addr: Addr, data: u8) {
        mem_map::write(self, addr, data);
    }
}
