use crate::{
    cart::cart::Cart,
    cpu::{
        exec::execute_next_instr,
        interrupt::try_handle_interrupts,
        regs::{CpuReg16, CpuReg8, CpuRegs},
    },
    debug::{self, debug_state},
    mem::{io_regs::IoReg, mem::Mem},
    other::joypad::handle_joypad_inputs,
    ppu::ppu::Ppu,
    time::{
        simple_clock::SimpleClock,
        timers::{
            update_timer_regs, CPU_PERIOD_MCYCLES, DIV_PERIOD_MCYCLES, TAC_CLK_0_PERIOD_MCYCLES,
        },
    },
};

pub struct Options {
    pub kill_on_infinite_loop: bool,
}

pub struct Sys {
    pub options: Options,

    pub mem: Mem,

    pub ppu: Ppu,

    pub regs: CpuRegs,

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
    pub fn new(options: Options, cart: Cart) -> Self {
        let mut sys = Self {
            options,

            mem: Mem::new(cart),

            ppu: Ppu::new(),

            regs: CpuRegs::new(),

            cpu_clock: SimpleClock::new("CPU", CPU_PERIOD_MCYCLES),
            div_timer_clock: SimpleClock::new("DIV", DIV_PERIOD_MCYCLES),
            tima_timer_clock: SimpleClock::new("TIMA", TAC_CLK_0_PERIOD_MCYCLES),

            cpu_delay_ticks: 0,

            cpu_enable: true,
            lcd_enable: true,
            interrupt_master_enable: false,

            hard_lock: false,
            is_render_pending: false,
        };

        Self::initialize(&mut sys);

        return sys;
    }

    fn initialize(sys: &mut Self) {
        // Set CPU registers to defaults.
        sys.regs.set_8(CpuReg8::A, 0x01);
        sys.regs.set_8(CpuReg8::F, 0b1000_0000);
        sys.regs.set_8(CpuReg8::B, 0x00);
        sys.regs.set_8(CpuReg8::C, 0x13);

        sys.regs.set_8(CpuReg8::D, 0x00);
        sys.regs.set_8(CpuReg8::E, 0xD8);
        sys.regs.set_8(CpuReg8::H, 0x01);
        sys.regs.set_8(CpuReg8::L, 0x48);

        sys.regs.set_16(CpuReg16::PC, 0x0100);
        sys.regs.set_16(CpuReg16::SP, 0xFFFE);

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

    pub fn run_one_m_cycle(&mut self) {
        if self.cpu_clock.update_and_check() {
            self.cpu_delay_ticks = u32::saturating_sub(self.cpu_delay_ticks, 1);
            if self.cpu_delay_ticks == 0 {
                try_handle_interrupts(self);
                if self.cpu_enable {
                    self.cpu_delay_ticks = execute_next_instr(self);
                }
            }
        }

        Ppu::update_ppu(self);
        update_timer_regs(self);
        handle_joypad_inputs(self);

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
            //debug::print_system_state(&self);
            self.hard_lock = true;
            return;
        }

        // self.test_code();

        //////////////////////////////////////////////////////////////

        return;
    }

    fn test_code(&mut self) {
        if debug_state().total_instrs_executed > 100 {
            self.hard_lock = true;
        }
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
