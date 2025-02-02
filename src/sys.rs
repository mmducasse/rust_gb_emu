use std::time::Instant;

use crate::{
    cart::cart::Cart,
    cpu::{
        exec::execute_next_instr,
        interrupt::try_handle_interrupts,
        regs::{CpuReg16, CpuReg8, CpuRegs},
    },
    debug::{self, debug_state},
    mem::{
        array::Array,
        io_regs::{IoReg, IoRegs},
        mem::Mem,
        sections::{self, Addr, MemSection},
    },
    ppu::ppu::Ppu,
    time::{
        real_clock::RealClock,
        simple_clock::SimpleClock,
        timers::{update_timer_regs, CPU_PERIOD_DOTS, DIV_PERIOD_DOTS, TAC_CLK_0_PERIOD_DOTS},
    },
    util::math::{bit8, set_bit8},
};

pub struct Sys {
    pub mem: Mem,

    pub ppu: Ppu,

    pub regs: CpuRegs,

    pub sys_clock: SimpleClock,
    pub cpu_clock: SimpleClock,
    pub div_timer_clock: SimpleClock,
    pub tima_timer_clock: SimpleClock,

    pub cpu_delay_ticks: u32,

    pub cpu_enable: bool,
    pub lcd_enable: bool,
    pub interrupt_master_enable: bool,

    pub hard_lock: bool,
    pub is_render_pending: bool,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            mem: Mem::new(),

            ppu: Ppu::new(),

            regs: CpuRegs::new(),

            sys_clock: SimpleClock::new("SYS", 1),
            cpu_clock: SimpleClock::new("CPU", CPU_PERIOD_DOTS),
            div_timer_clock: SimpleClock::new("DIV", DIV_PERIOD_DOTS),
            tima_timer_clock: SimpleClock::new("TIMA", TAC_CLK_0_PERIOD_DOTS),

            cpu_delay_ticks: 0,

            cpu_enable: true,
            lcd_enable: true,
            interrupt_master_enable: true,

            hard_lock: false,
            is_render_pending: false,
        }
    }

    pub fn initialize(sys: &mut Self) {
        // Set CPU registers to defaults.
        sys.regs.set_8(CpuReg8::A, 0x01);
        sys.regs.set_8(CpuReg8::F, 0b1000_0000);
        sys.regs.set_8(CpuReg8::B, 0x00);
        sys.regs.set_8(CpuReg8::C, 0x13);

        sys.regs.set_8(CpuReg8::D, 0x00);
        sys.regs.set_8(CpuReg8::E, 0xD8);
        sys.regs.set_8(CpuReg8::H, 0x01);
        sys.regs.set_8(CpuReg8::L, 0x48);

        sys.set_pc(0x0100);
        sys.set_sp(0xFFFE);

        // Set IO registers to defaults.
        use IoReg::*;
        sys.mem.io_regs.set(P1, 0xCF);
        sys.mem.io_regs.set(Sb, 0x00);
        sys.mem.io_regs.set(Sc, 0x7E);
        sys.mem.io_regs.set(Div, 0xAB);
        sys.mem.io_regs.set(Tima, 0x00);
        sys.mem.io_regs.set(Tma, 0x00);
        sys.mem.io_regs.set(Tac, 0xF8);
        sys.mem.io_regs.set(If, 0xE1);
        sys.mem.io_regs.set(Lcdc, 0x91);
        sys.mem.io_regs.set(Stat, 0x85);
        sys.mem.io_regs.set(Scy, 0x00);
        sys.mem.io_regs.set(Scx, 0x00);
        sys.mem.io_regs.set(Ly, 0x00);
        sys.mem.io_regs.set(Lyc, 0x00);
        sys.mem.io_regs.set(Dma, 0xFF);
        sys.mem.io_regs.set(Bgp, 0xFC);
        sys.mem.io_regs.set(Obp0, 0);
        sys.mem.io_regs.set(Obp1, 0);
        sys.mem.io_regs.set(Wy, 0x00);
        sys.mem.io_regs.set(Wx, 0x00);

        // Key1..Svbk are not initialized.

        sys.mem.io_regs.set(Ie, 0x00);
    }

    pub fn run(&mut self) {
        while !self.hard_lock {
            self.run_one_m_cycle();
        }
    }

    pub fn run_one_m_cycle(&mut self) -> bool {
        let mut did_run_cpu_instr = false;

        for i in 0..4 {
            self.sys_clock.update_and_check();

            update_timer_regs(self);
        }

        if self.cpu_clock.update_and_check() {
            self.cpu_delay_ticks = u32::saturating_sub(self.cpu_delay_ticks, 1);
        }
        if self.cpu_delay_ticks == 0 {
            try_handle_interrupts(self);
            self.cpu_delay_ticks = execute_next_instr(self);
            did_run_cpu_instr = true;
        }

        Ppu::update_ppu(self);

        ///////// DEBUG //////////////////////////////////////////////
        if let Some(kill_after_nop_count) = debug_state().config.kill_after_nop_count {
            if debug_state().nop_count >= kill_after_nop_count {
                debug::fail("Debug max NOP count exceeded.");
            }
        }

        if let Some(kill_after_ticks) = debug_state().config.kill_after_cpu_ticks {
            if self.cpu_clock.debug_total_ticks >= kill_after_ticks {
                debug::fail("Debug kill time elapsed.");
            }
        }

        if let Some(failure) = debug::get_failure() {
            println!("FAILURE: {}", failure);
            debug::print_system_state(&self);
            self.hard_lock = true;
            return did_run_cpu_instr;
        }

        // self.test_code();

        //////////////////////////////////////////////////////////////

        return did_run_cpu_instr;
    }

    fn test_code(&mut self) {
        if debug_state().total_instrs_executed > 100 {
            self.hard_lock = true;
        }
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
        println!("IE={:0>8b}", self.mem.io_regs.get(IoReg::Ie));
        println!("IF={:0>8b}", self.mem.io_regs.get(IoReg::If));

        Ppu::print(self);

        self.cpu_clock.print();
        self.div_timer_clock.print();
        self.tima_timer_clock.print();
    }
}
