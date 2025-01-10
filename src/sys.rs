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
    ppu::ppu::Ppu,
    time::{
        clock::Clock,
        timers::{update_timer_regs, CPU_FREQ_HZ, DIV_FREQ_HZ, LCD_FREQ_HZ, TAC_CLK_SEL_0_FREQ_HZ},
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
    pub io_regs: IoRegs,
    pub hram: Ram,
    pub ie_reg: Ram,

    pub ppu: Ppu,

    pub cpu_clock: Clock,
    pub div_timer_clock: Clock,
    pub tima_timer_clock: Clock,

    pub cpu_enable: bool,
    pub lcd_enable: bool,
    pub interrupt_master_enable: bool,

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

            ppu: Ppu::new(),

            cpu_clock: Clock::new("CPU", CPU_FREQ_HZ),
            div_timer_clock: Clock::new("DIV", DIV_FREQ_HZ),
            tima_timer_clock: Clock::new("TIMA", TAC_CLK_SEL_0_FREQ_HZ),

            cpu_enable: true,
            lcd_enable: true,
            interrupt_master_enable: true,

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
        sys.mem_set(P1.addr(), 0xCF);
        sys.mem_set(Sb.addr(), 0x00);
        sys.mem_set(Sc.addr(), 0x7E);
        sys.mem_set(Div.addr(), 0x18);
        sys.mem_set(Tima.addr(), 0x00);
        sys.mem_set(Tma.addr(), 0x00);
        sys.mem_set(Tac.addr(), 0xF8);
        sys.mem_set(If.addr(), 0xE1);
        sys.mem_set(Lcdc.addr(), 0x91);
        sys.mem_set(Stat.addr(), 0x81);
        sys.mem_set(Scy.addr(), 0x00);
        sys.mem_set(Scx.addr(), 0x00);
        sys.mem_set(Ly.addr(), 0x91);
        sys.mem_set(Lyc.addr(), 0x00);
        sys.mem_set(Dma.addr(), 0xFF);
        sys.mem_set(Bgp.addr(), 0xFC);
        sys.mem_set(Obp0.addr(), 0);
        sys.mem_set(Obp1.addr(), 0);
        sys.mem_set(Wy.addr(), 0x00);
        sys.mem_set(Wx.addr(), 0x00);

        // Key1..Svbk are not initialized.

        sys.mem_set(Ie.addr(), 0x00);
    }

    pub fn run(&mut self) {
        let mut prev = Instant::now();
        while !self.hard_lock {
            let now = Instant::now();
            // println!("Iter: {:?}", now);

            let elapsed_s = (now - prev).as_secs_f64();
            let dots = self.cpu_clock.update(elapsed_s);
            update_timer_regs(self, elapsed_s);
            Ppu::update_ppu(self, dots);
            try_handle_interrupts(self);
            execute_next_instr(self);

            if self.debug.nop_count > Debug::EXIT_AFTER_NOP_COUNT {
                break;
            }

            if let Some(kill_after_seconds) = self.debug.kill_after_seconds {
                if kill_after_seconds < 0.0 {
                    //Debug::fail(self, "Debug kill time elapsed.");
                    return;
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

    pub fn mem_get(&self, addr: Addr) -> u8 {
        map::read(self, addr)
    }

    pub fn mem_get_bit(&self, addr: Addr, idx: u8) -> u8 {
        let data = self.mem_get(addr);
        return bit8(&data, idx);
    }

    pub fn get_hl_p(&self) -> u8 {
        let addr = self.regs.get_16(CpuReg16::HL);
        self.mem_get(addr)
    }

    pub fn mem_set(&mut self, addr: Addr, data: u8) {
        map::write(self, addr, data);
    }

    pub fn mem_set_bit(&mut self, addr: Addr, idx: u8, value: u8) {
        let mut data = self.mem_get(addr);
        set_bit8(&mut data, idx, value);
        self.mem_set(addr, data);
    }

    pub fn mem_mut(&mut self, addr: impl Into<Addr>, mut f: impl FnMut(&mut u8) -> ()) -> u8 {
        let addr = addr.into();
        let mut data = self.mem_get(addr);
        f(&mut data);
        self.mem_set(addr, data);

        return data;
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
        println!("IE={:0>8b}", self.mem_get(IoRegId::Ie.addr()));
        println!("IF={:0>8b}", self.mem_get(IoRegId::If.addr()));

        Ppu::print(self);

        self.cpu_clock.print();
        self.div_timer_clock.print();
        self.tima_timer_clock.print();
    }
}
