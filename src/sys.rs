use std::time::Instant;

use crate::{
    cpu::{
        exec::execute_next_instr,
        interrupt::try_handle_interrupts,
        regs::{CpuReg16, CpuRegs},
        timer::{update_timer_regs, TimerData},
    },
    debug::Debug,
    mem::{
        cart::Cart,
        map::{self, Addr, MemSection},
        ram::Ram,
    },
    util::math::{bit8, set_bit8},
};

pub struct Sys {
    pub cart: Cart,
    pub regs: CpuRegs,
    pub wram: Ram,
    pub vram: Ram,
    pub ext_ram: Ram,
    pub oam: Ram,
    pub io_regs: Ram,
    pub hram: Ram,
    pub ie_reg: Ram,

    pub cpu_enable: bool,
    pub lcd_enable: bool,
    pub interrupt_master_enable: bool,
    pub timer_data: TimerData,

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
            oam: Ram::new(MemSection::Oam.size()),
            io_regs: Ram::new(MemSection::IoRegs.size()),
            hram: Ram::new(MemSection::Hram.size()),
            ie_reg: Ram::new(MemSection::IeReg.size()),

            cpu_enable: true,
            lcd_enable: true,
            interrupt_master_enable: true,
            timer_data: TimerData::new(),

            hard_lock: false,
            debug: Debug::new(),
        }
    }

    pub fn run(&mut self) {
        let mut prev = Instant::now();
        while !self.hard_lock {
            let now = Instant::now();
            // println!("Iter: {:?}", now);

            let elapsed = now - prev;
            update_timer_regs(self, elapsed);
            try_handle_interrupts(self);
            execute_next_instr(self);

            if self.debug.nop_count > Debug::EXIT_AFTER_NOP_COUNT {
                break;
            }

            self.test_code();

            prev = now;
        }
    }

    fn test_code(&mut self) {
        if self.debug.total_instrs_executed > 100 {
            self.hard_lock = true;
        }
    }

    pub fn rd_mem(&self, addr: Addr) -> u8 {
        map::read(self, addr)
    }

    pub fn rd_mem_bit(&self, addr: Addr, idx: u8) -> u8 {
        let data = self.rd_mem(addr);
        return bit8(&data, idx);
    }

    pub fn rd_hl_p(&self) -> u8 {
        let addr = self.regs.get_16(CpuReg16::HL);
        self.rd_mem(addr)
    }

    pub fn wr_mem(&mut self, addr: Addr, data: u8) {
        map::write(self, addr, data);
    }

    pub fn wr_mem_bit(&mut self, addr: Addr, idx: u8, value: u8) {
        let mut data = self.rd_mem(addr);
        set_bit8(&mut data, idx, value);
        self.wr_mem(addr, data);
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
