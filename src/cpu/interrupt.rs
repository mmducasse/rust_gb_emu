use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    debug,
    mem::{io_regs::IoReg, sections::Addr},
    sys::Sys,
    util::math::{bit8, set_bit8},
};

use super::exec::call;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Debug)]
pub enum InterruptType {
    VBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl InterruptType {
    pub fn jump_addr(self) -> Addr {
        return match self {
            InterruptType::VBlank => 0x40,
            InterruptType::Stat => 0x48,
            InterruptType::Timer => 0x50,
            InterruptType::Serial => 0x58,
            InterruptType::Joypad => 0x60,
        };
    }
}

impl InterruptType {
    pub fn flag_idx(self) -> u8 {
        return self as u8;
    }
}

pub fn request_interrupt(sys: &mut Sys, type_: InterruptType) {
    //println!("Int req: {:?}", type_);
    sys.mem.io_regs.mut_(IoReg::If, |if_| {
        set_bit8(if_, type_.flag_idx(), 1);
    });
}

pub fn try_handle_interrupts(sys: &mut Sys) {
    if !sys.interrupt_master_enable {
        return;
    }

    let ie = sys.mem.io_regs.get(IoReg::Ie);
    let if_ = sys.mem.io_regs.get(IoReg::If);
    for type_ in InterruptType::iter() {
        let flag_idx = type_.flag_idx();
        let is_int_enabled = bit8(&ie, flag_idx) == 1;
        let is_int_requested = bit8(&if_, flag_idx) == 1;

        let force = type_ == InterruptType::VBlank;

        if (is_int_enabled && is_int_requested) || force {
            handle_interrupt(sys, type_);
            return; // Only handle highest priority requested interrupt.
        }
    }
}

fn handle_interrupt(sys: &mut Sys, type_: InterruptType) {
    println!("Handling INT: {:?}", type_);

    sys.interrupt_master_enable = false;

    let flag_idx = type_.flag_idx();
    sys.mem.io_regs.mut_(IoReg::If, |if_| {
        set_bit8(if_, flag_idx, 0);
    });

    // 2 NOP cycles
    sys.cpu_delay_ticks += 2;

    let prev_pc = sys.get_pc();
    let next_pc = type_.jump_addr();
    call(sys, prev_pc, next_pc); // 3 cycles

    sys.cpu_delay_ticks += 3;

    debug::debug_state().print_instrs = 10;
}
