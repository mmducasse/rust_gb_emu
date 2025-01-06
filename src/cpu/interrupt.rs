use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    mem::{io_regs::IoRegId, map::Addr},
    sys::Sys,
    util::math::bit8,
};

use super::exec::call;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Debug)]
pub enum InterruptType {
    VBlank,
    Lcd,
    Timer,
    Serial,
    Joypad,
}

impl InterruptType {
    pub fn jump_addr(self) -> Addr {
        return match self {
            InterruptType::VBlank => 0x40,
            InterruptType::Lcd => 0x48,
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

pub fn try_handle_interrupts(sys: &mut Sys) {
    if !sys.interrupt_master_enable {
        return;
    }

    let ie = sys.rd_mem(IoRegId::Ie.addr());
    let if_ = sys.rd_mem(IoRegId::If.addr());
    for type_ in InterruptType::iter() {
        let flag_idx = type_.flag_idx();
        let is_int_enabled = bit8(&ie, flag_idx) == 1;
        let is_int_requested = bit8(&if_, flag_idx) == 1;

        if is_int_enabled && is_int_requested {
            handle_interrupt(sys, type_);
            return; // Only handle highest priority requested interrupt.
        }
    }
}

fn handle_interrupt(sys: &mut Sys, type_: InterruptType) {
    sys.interrupt_master_enable = false;

    let flag_idx = type_.flag_idx();
    sys.wr_mem_bit(IoRegId::If.addr(), flag_idx, 0);

    // 2 NOP cycles

    let prev_pc = sys.get_pc();
    let next_pc = type_.jump_addr();
    call(sys, prev_pc, next_pc); // 3 cycles
}
