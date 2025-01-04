use crate::{
    cpu::{
        exec::execute_next_instr,
        regs::{CpuReg16, CpuRegs},
    },
    debug::Debug,
    mem::{
        cart::Cart,
        map::{self, Addr, MemSection},
        ram::Ram,
    },
};

pub struct Sys {
    pub cart: Cart,
    pub regs: CpuRegs,
    pub wram: Ram,
    pub vram: Ram,
    pub ext_ram: Ram,

    pub hard_lock: bool,
    pub debug: Debug,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            cart: Cart::new(),
            regs: CpuRegs::new(),
            wram: Ram::new(MemSection::Wram.size()),
            vram: Ram::new(MemSection::Vram.size()),
            ext_ram: Ram::new(MemSection::ExtRam.size()),

            hard_lock: false,
            debug: Debug::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.hard_lock {
            execute_next_instr(self);
            if self.debug.nop_count > Debug::EXIT_AFTER_NOP_COUNT {
                break;
            }
        }
    }

    pub fn rd_mem(&self, addr: Addr) -> u8 {
        map::read(self, addr)
    }

    pub fn rd_hl_p(&self) -> u8 {
        let addr = self.regs.get_16(CpuReg16::HL);
        self.rd_mem(addr)
    }

    pub fn wr_mem(&mut self, addr: Addr, data: u8) {
        map::write(self, addr, data);
    }

    pub fn get_pc(&self) -> Addr {
        return self.regs.get_16(CpuReg16::PC);
    }

    pub fn set_pc(&mut self, addr: Addr) {
        self.regs.set_16(CpuReg16::PC, addr);
    }

    pub fn inc_pc(&mut self) {
        let mut pc = self.get_pc();
        pc = u16::wrapping_add(pc, 1);
        self.set_pc(pc);
    }

    pub fn get_sp(&self) -> Addr {
        return self.regs.get_16(CpuReg16::SP);
    }

    pub fn set_sp(&mut self, addr: Addr) {
        self.regs.set_16(CpuReg16::SP, addr);
    }

    pub fn inc_sp(&mut self) {
        let mut sp = self.get_sp();
        sp = u16::wrapping_add(sp, 1);
        self.regs.set_16(CpuReg16::SP, sp);
    }

    pub fn dec_sp(&mut self) {
        let mut sp = self.get_sp();
        sp = u16::wrapping_sub(sp, 1);
        self.regs.set_16(CpuReg16::SP, sp);
    }
}
