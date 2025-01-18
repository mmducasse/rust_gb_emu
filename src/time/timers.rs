use std::time::Duration;

use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    mem::io_regs::IoReg,
    sys::Sys,
    util::math::{bit8, bits8},
};

// pub const CPU_FREQ_HZ: f64 = 4.194304E6;
// pub const LCD_FREQ_HZ: f64 = 59.73;

// pub const DIV_FREQ_HZ: f64 = 16384.0;
// pub const TAC_CLK_SEL_0_FREQ_HZ: f64 = 4194.0;
// pub const TAC_CLK_SEL_1_FREQ_HZ: f64 = 268400.0;
// pub const TAC_CLK_SEL_2_FREQ_HZ: f64 = 67110.0;
// pub const TAC_CLK_SEL_3_FREQ_HZ: f64 = 16780.0;

pub const CPU_PERIOD_DOTS: u32 = 4;
pub const DIV_PERIOD_DOTS: u32 = 256;
pub const TAC_CLK_0_PERIOD_DOTS: u32 = 256;
pub const TAC_CLK_1_PERIOD_DOTS: u32 = 16;
pub const TAC_CLK_2_PERIOD_DOTS: u32 = 64;
pub const TAC_CLK_3_PERIOD_DOTS: u32 = 128;

pub fn update_timer_regs(sys: &mut Sys) {
    // DIV: incs every 16384 Hz; Writing any sets to 0x00; reset on STOP; doesnt tick until stop mode ends)
    // TIMA: incs at freq specified in TAC; when overflows, it is reset to value in TMA and an interrupt is reqd
    // TMA: determines TIMA reset value after overflow
    // TAC: .2: enable; .1-0: clock select;

    let div_ticked = sys.div_timer_clock.update_and_check();

    let div = sys.mem_get(IoReg::Div);
    let div_ = u8::wrapping_add(div, 1);
    if div_ < div {
        // DIV overflow
        println!("DIV overflow");
    }
    sys.mem_set(IoReg::Div, div_);

    // Update TIMA
    let tac = sys.mem_get(IoReg::Tac);
    let enable = bit8(&tac, 2); // todo unused
    let clock_sel = bits8(&tac, 1, 0);
    let tima_clk_period = match clock_sel {
        0 => TAC_CLK_0_PERIOD_DOTS,
        1 => TAC_CLK_1_PERIOD_DOTS,
        2 => TAC_CLK_2_PERIOD_DOTS,
        3 => TAC_CLK_3_PERIOD_DOTS,
        _ => unreachable!(),
    };

    sys.tima_timer_clock.set_period_dots(tima_clk_period);

    let tima_ticked = sys.tima_timer_clock.update_and_check();

    if tima_ticked {
        let tima = sys.mem_get(IoReg::Tima);
        let tima_ = u8::wrapping_add(tima, 1);
        if tima_ < tima {
            // TIMA overflow
            println!("TIMA overflow");
            let tma = sys.mem_get(IoReg::Tma);
            sys.mem_set(IoReg::Div, tma);
            request_interrupt(sys, InterruptType::Timer);
        } else {
            sys.mem_set(IoReg::Div, tima_);
        }
    }
}
