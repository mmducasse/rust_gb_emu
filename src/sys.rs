use std::time::Instant;

use crate::{
    cpu::{
        exec::execute_next_instr,
        interrupt::try_handle_interrupts,
        regs::{CpuReg16, CpuReg8, CpuRegs},
    },
    debug::Debug,
    mem::{
        cart::Cart,
        io_regs::{IoRegId, IoRegs},
        map::{self, Addr, MemSection},
        ram::Ram,
    },
    time::{update_timer_regs, TimerData},
    util::math::{bit8, set_bit8},
};

pub struct Sys {
    pub cart: Cart,
    pub regs: CpuRegs,
    pub wram: Ram,
    pub vram: Ram,
    pub ext_ram: Ram,
    pub oam: Ram,
    pub io_regs: IoRegs,
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
            io_regs: IoRegs::new(),
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

    pub fn initialize(sys: &mut Self) {
        // Set CPU registers to defaults.
        sys.regs.set_8(CpuReg8::A, 0x01);
        sys.regs.set_8(CpuReg8::F, 0x00);
        sys.regs.set_8(CpuReg8::B, 0xFF);
        sys.regs.set_8(CpuReg8::C, 0x13);

        sys.regs.set_8(CpuReg8::D, 0x00);
        sys.regs.set_8(CpuReg8::E, 0xC1);
        sys.regs.set_8(CpuReg8::H, 0x84);
        sys.regs.set_8(CpuReg8::L, 0x03);

        sys.set_pc(0x0100);
        sys.set_sp(0xFFFE);

        // Set IO registers to defaults.
        use IoRegId::*;
        sys.wr_mem(P1.addr(), 0xCF);
        sys.wr_mem(Sb.addr(), 0x00);
        sys.wr_mem(Sc.addr(), 0x7E);
        sys.wr_mem(Div.addr(), 0x18);
        sys.wr_mem(Tima.addr(), 0x00);
        sys.wr_mem(Tma.addr(), 0x00);
        sys.wr_mem(Tac.addr(), 0xF8);
        sys.wr_mem(If.addr(), 0xE1);
        sys.wr_mem(Lcdc.addr(), 0x91);
        sys.wr_mem(Stat.addr(), 0x81);
        sys.wr_mem(Scy.addr(), 0x00);
        sys.wr_mem(Scx.addr(), 0x00);
        sys.wr_mem(Ly.addr(), 0x91);
        sys.wr_mem(Lyc.addr(), 0x00);
        sys.wr_mem(Dma.addr(), 0xFF);
        sys.wr_mem(Bgp.addr(), 0xFC);
        sys.wr_mem(Obp0.addr(), 0);
        sys.wr_mem(Obp1.addr(), 0);
        sys.wr_mem(Wy.addr(), 0x00);
        sys.wr_mem(Wx.addr(), 0x00);

        // Key1..Svbk are not initialized.

        sys.wr_mem(Ie.addr(), 0x00);
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

            if let Some(kill_after_seconds) = self.debug.kill_after_seconds {
                if kill_after_seconds < 0.0 {
                    Debug::fail(self, "Debug kill time elapsed.");
                }
            }

            // self.test_code();

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

    pub fn print(&self) {
        self.regs.print();
        println!("IME={}", self.interrupt_master_enable);
        println!("IE={:0>8b}", self.rd_mem(IoRegId::Ie.addr()));
        println!("IF={:0>8b}", self.rd_mem(IoRegId::If.addr()));
    }
}
