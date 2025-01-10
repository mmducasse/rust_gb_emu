use std::time::Duration;

use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    mem::io_regs::IoReg,
    sys::Sys,
    util::math::{bit8, bits8},
};

pub const CPU_FREQ_HZ: f64 = 4.194304E6;
pub const LCD_FREQ_HZ: f64 = 59.73;

pub const DIV_FREQ_HZ: f64 = 16384.0;
pub const TAC_CLK_SEL_0_FREQ_HZ: f64 = 4194.0;
pub const TAC_CLK_SEL_1_FREQ_HZ: f64 = 268400.0;
pub const TAC_CLK_SEL_2_FREQ_HZ: f64 = 67110.0;
pub const TAC_CLK_SEL_3_FREQ_HZ: f64 = 16780.0;

pub fn update_timer_regs(sys: &mut Sys, elapsed_s: f64) {
    // DIV: incs every 16384 Hz; Writing any sets to 0x00; reset on STOP; doesnt tick until stop mode ends)
    // TIMA: incs at freq specified in TAC; when overflows, it is reset to value in TMA and an interrupt is reqd
    // TMA: determines TIMA reset value after overflow
    // TAC: .2: enable; .1-0: clock select;

    let div_ticks = sys.div_timer_clock.update(elapsed_s);

    let div = sys.mem_get(IoReg::Div);
    let div_ = u8::wrapping_add(div, div_ticks.clamp(0x00, 0xFF) as u8);
    if div_ < div {
        // DIV overflow
        println!("DIV overflow");
    }
    sys.mem_set(IoReg::Div, div_);

    // Update TIMA
    let tac = sys.mem_get(IoReg::Tac);
    let enable = bit8(&tac, 2); // todo unused
    let clock_sel = bits8(&tac, 1, 0);
    let tima_clk_freq_hz = match clock_sel {
        0 => TAC_CLK_SEL_0_FREQ_HZ,
        1 => TAC_CLK_SEL_1_FREQ_HZ,
        2 => TAC_CLK_SEL_2_FREQ_HZ,
        3 => TAC_CLK_SEL_3_FREQ_HZ,
        _ => unreachable!(),
    };

    sys.tima_timer_clock.set_frequency_hz(tima_clk_freq_hz);

    let tima_ticks = sys.tima_timer_clock.update(elapsed_s);

    let tima = sys.mem_get(IoReg::Tima);
    let tima_ = u8::wrapping_add(tima, tima_ticks.clamp(0x00, 0xFF) as u8);
    if tima_ < tima {
        // TIMA overflow
        println!("TIMA overflow");
        let tma = sys.mem_get(IoReg::Tma);
        sys.mem_set(IoReg::Div, tma);
        request_interrupt(sys, InterruptType::Timer);
    } else {
        sys.mem_set(IoReg::Div, tima_);
    }

    // Other
    if let Some(kill_after_seconds) = sys.debug.kill_after_seconds.as_mut() {
        *kill_after_seconds -= elapsed_s;
    }
}
