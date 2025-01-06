use std::time::Duration;

use crate::{
    mem::io_regs::IoRegId,
    sys::Sys,
    util::math::{bit8, bits8},
};

const DIV_FREQ_HZ: f64 = 16384.0;
const TAC_CLK_SEL_0_FREQ_HZ: f64 = 4194.0;
const TAC_CLK_SEL_1_FREQ_HZ: f64 = 268400.0;
const TAC_CLK_SEL_2_FREQ_HZ: f64 = 67110.0;
const TAC_CLK_SEL_3_FREQ_HZ: f64 = 16780.0;

const DIV_CLK_PERIOD: f64 = 1.0 / DIV_FREQ_HZ;
const TAC_CLK_SEL_0_CLK_PERIOD: f64 = 1.0 / TAC_CLK_SEL_0_FREQ_HZ;
const TAC_CLK_SEL_1_CLK_PERIOD: f64 = 1.0 / TAC_CLK_SEL_1_FREQ_HZ;
const TAC_CLK_SEL_2_CLK_PERIOD: f64 = 1.0 / TAC_CLK_SEL_2_FREQ_HZ;
const TAC_CLK_SEL_3_CLK_PERIOD: f64 = 1.0 / TAC_CLK_SEL_3_FREQ_HZ;

pub struct TimerData {
    pub div_time_since_last_tick: Duration,
    pub tima_time_since_last_tick: Duration,
}

impl TimerData {
    pub fn new() -> Self {
        Self {
            div_time_since_last_tick: Duration::ZERO,
            tima_time_since_last_tick: Duration::ZERO,
        }
    }
}

pub fn update_timer_regs(sys: &mut Sys, elapsed: Duration) {
    // DIV: incs every 16384 Hz; Writing any sets to 0x00; reset on STOP; doesnt tick until stop mode ends)
    // TIMA: incs at freq specified in TAC; when overflows, it is reset to value in TMA and an interrupt is reqd
    // TMA: determines TIMA reset value after overflow
    // TAC: .2: enable; .1-0: clock select;

    // Update DIV
    let div_time_since_last_tick =
        (sys.timer_data.div_time_since_last_tick + elapsed).as_secs_f64();
    let div_ticks = div_time_since_last_tick / DIV_CLK_PERIOD;
    let div_time_rem = (div_ticks * DIV_CLK_PERIOD) - div_time_since_last_tick;
    sys.timer_data.div_time_since_last_tick = Duration::from_secs_f64(div_time_rem.max(0.0));

    let div = sys.rd_mem(IoRegId::Div.addr());
    let div_ = u8::wrapping_add(div, div_ticks.clamp(0.0, 255.0) as u8);
    if div_ < div {
        // DIV overflow
        println!("DIV overflow");
    }
    sys.wr_mem(IoRegId::Div.addr(), div_);

    // Update TIMA
    let tac = sys.rd_mem(IoRegId::Tac.addr());
    let enable = bit8(&tac, 2); // todo unused
    let clock_sel = bits8(&tac, 1, 0);
    let tima_clk_period = match clock_sel {
        0 => TAC_CLK_SEL_0_CLK_PERIOD,
        1 => TAC_CLK_SEL_1_CLK_PERIOD,
        2 => TAC_CLK_SEL_2_CLK_PERIOD,
        3 => TAC_CLK_SEL_3_CLK_PERIOD,
        _ => unreachable!(),
    };

    let tima_time_since_last_tick =
        (sys.timer_data.tima_time_since_last_tick + elapsed).as_secs_f64();
    let tima_ticks = tima_time_since_last_tick / tima_clk_period;
    let tima_time_rem = (tima_ticks * tima_clk_period) - tima_time_since_last_tick;
    sys.timer_data.tima_time_since_last_tick = Duration::from_secs_f64(tima_time_rem.max(0.0));

    let tima = sys.rd_mem(IoRegId::Tima.addr());
    let tima_ = u8::wrapping_add(tima, tima_ticks.clamp(0.0, 255.0) as u8);
    if tima_ < tima {
        // TIMA overflow
        println!("TIMA overflow");
        let tma = sys.rd_mem(IoRegId::Tma.addr());
        sys.wr_mem(IoRegId::Div.addr(), tma);
        // todo request Timer Interrupt
    } else {
        sys.wr_mem(IoRegId::Div.addr(), tima_);
    }

    // println!("DIV={}  TIMA={}", div_, tima_);

    // Other
    if let Some(kill_after_seconds) = sys.debug.kill_after_seconds.as_mut() {
        *kill_after_seconds -= elapsed.as_secs_f64();
    }
}
