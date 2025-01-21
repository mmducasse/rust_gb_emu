use std::time::Instant;

use crate::{
    cart::cart::Cart,
    cpu::{
        exec::execute_next_instr,
        interrupt::try_handle_interrupts,
        regs::{CpuReg16, CpuReg8, CpuRegs},
    },
    debug::debug_state,
    mem::{
        array::Array,
        io_regs::{IoReg, IoRegs},
        map::{self, Addr, MemSection},
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
    pub cart: Cart,
    pub regs: CpuRegs,
    pub wram: Array,
    pub vram: Array,
    pub oam: Array,
    pub io_regs: IoRegs,
    pub hram: Array,
    pub ie_reg: Array,

    pub ppu: Ppu,

    pub sys_clock: SimpleClock,
    pub cpu_clock: SimpleClock,
    pub div_timer_clock: SimpleClock,
    pub tima_timer_clock: SimpleClock,

    pub cpu_delay_ticks: u32,

    pub cpu_enable: bool,
    pub lcd_enable: bool,
    pub interrupt_master_enable: bool,

    pub hard_lock: bool,
}

impl Sys {
    pub fn new() -> Self {
        Self {
            cart: Cart::new(),
            regs: CpuRegs::new(),
            wram: Array::from_mem_section(MemSection::Wram),
            vram: Array::from_mem_section(MemSection::Vram),
            oam: Array::from_mem_section(MemSection::Oam),
            io_regs: IoRegs::new(),
            hram: Array::from_mem_section(MemSection::Hram),
            ie_reg: Array::from_mem_section(MemSection::IeReg),

            ppu: Ppu::new(),

            sys_clock: SimpleClock::new("SYS", 1),
            cpu_clock: SimpleClock::new("CPU", CPU_PERIOD_DOTS),
            div_timer_clock: SimpleClock::new("DIV", DIV_PERIOD_DOTS),
            tima_timer_clock: SimpleClock::new("TIMA", TAC_CLK_0_PERIOD_DOTS),

            cpu_delay_ticks: 0,

            cpu_enable: true,
            lcd_enable: true,
            interrupt_master_enable: true,

            hard_lock: false,
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
        use IoReg::*;
        sys.set_io_reg(P1, 0xCF);
        sys.set_io_reg(Sb, 0x00);
        sys.set_io_reg(Sc, 0x7E);
        sys.set_io_reg(Div, 0x18);
        sys.set_io_reg(Tima, 0x00);
        sys.set_io_reg(Tma, 0x00);
        sys.set_io_reg(Tac, 0xF8);
        sys.set_io_reg(If, 0xE1);
        sys.set_io_reg(Lcdc, 0x91);
        sys.set_io_reg(Stat, 0x81);
        sys.set_io_reg(Scy, 0x00);
        sys.set_io_reg(Scx, 0x00);
        sys.set_io_reg(Ly, 0x91);
        sys.set_io_reg(Lyc, 0x00);
        sys.set_io_reg(Dma, 0xFF);
        sys.set_io_reg(Bgp, 0xFC);
        sys.set_io_reg(Obp0, 0);
        sys.set_io_reg(Obp1, 0);
        sys.set_io_reg(Wy, 0x00);
        sys.set_io_reg(Wx, 0x00);

        // Key1..Svbk are not initialized.

        sys.mem_set(Ie, 0x00);
    }

    pub fn run(&mut self) {
        while !self.hard_lock {
            self.run_one();
        }
    }

    pub fn run_one(&mut self) {
        self.sys_clock.update_and_check();

        update_timer_regs(self);

        if self.cpu_clock.update_and_check() {
            self.cpu_delay_ticks = u32::saturating_sub(self.cpu_delay_ticks, 1);
        }
        if self.cpu_delay_ticks == 0 {
            self.cpu_delay_ticks = execute_next_instr(self);
            try_handle_interrupts(self);
        }

        Ppu::update_ppu(self);

        ///////// DEBUG //////////////////////////////////////////////
        if let Some(kill_after_nop_count) = debug_state().config.kill_after_nop_count {
            if debug_state().nop_count >= kill_after_nop_count {
                self.hard_lock = true;
                return;
            }
        }

        if let Some(kill_after_ticks) = debug_state().config.kill_after_cpu_ticks {
            if self.cpu_clock.debug_total_ticks >= kill_after_ticks {
                //Debug::fail(self, "Debug kill time elapsed.");
                self.hard_lock = true;
                return;
            }
        }

        // self.test_code();

        //////////////////////////////////////////////////////////////
    }

    fn test_code(&mut self) {
        if debug_state().total_instrs_executed > 100 {
            self.hard_lock = true;
        }
    }

    pub fn mem_get(&self, addr: impl Into<Addr>) -> u8 {
        let addr = addr.into();
        map::read(self, addr)
    }

    pub fn mem_get_bit(&self, addr: impl Into<Addr>, idx: u8) -> u8 {
        let addr = addr.into();
        let data = self.mem_get(addr);
        return bit8(&data, idx);
    }

    pub fn get_hl_p(&self) -> u8 {
        let addr = self.regs.get_16(CpuReg16::HL);
        self.mem_get(addr)
    }

    pub fn mem_set(&mut self, addr: impl Into<Addr>, data: u8) {
        let addr = addr.into();
        map::write(self, addr, data);
    }

    pub fn mem_set_bit(&mut self, addr: impl Into<Addr>, idx: u8, value: u8) {
        let addr = addr.into();
        let mut data = self.mem_get(addr);
        set_bit8(&mut data, idx, value);
        self.mem_set(addr, data);
    }

    // /// Returns a mutable reference to the data at 'addr'. Does not allow setting read-only bits.
    // /// To set read-only bits, call 'io_reg_mut'.
    // pub fn mem_mut(&mut self, addr: impl Into<Addr>, mut f: impl FnMut(&mut u8) -> ()) -> u8 {
    //     let addr = addr.into();
    //     let mut data = self.mem_get(addr);
    //     f(&mut data);
    //     self.mem_set(addr, data);

    //     return data;
    // }

    // /// Gets the entire byte in the IO reg. Doesn't abide by it's read mask.
    // pub fn get_io_reg(&self, reg: IoReg) -> u8 {
    //     return self.io_regs.get(reg);
    // }

    /// Sets the entire byte in the IO reg. Doesn't abide by it's write mask.
    pub fn set_io_reg(&mut self, reg: IoReg, data: u8) {
        *self.io_regs.mut_(reg) = data;
    }

    /// Applies a function to the byte in the IO reg. Doesn't abide by it's read/write masks.
    pub fn io_reg_mut(&mut self, reg: IoReg, mut f: impl FnMut(&mut u8) -> ()) -> u8 {
        let data = self.io_regs.mut_(reg);
        f(data);

        return *data;
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
        println!("IE={:0>8b}", self.mem_get(IoReg::Ie));
        println!("IF={:0>8b}", self.mem_get(IoReg::If));

        Ppu::print(self);

        self.cpu_clock.print();
        self.div_timer_clock.print();
        self.tima_timer_clock.print();
    }
}
