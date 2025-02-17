use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    mem::io_regs::IoReg,
    sys::Sys,
    util::math::{bit8, set_bit8},
};

use super::{
    consts::VIEWPORT_ORG,
    dma::{update_dma, Dma},
    render::render_scanline,
};

pub const DOTS_PER_SCANLINE: u32 = 456;
pub const SCANLINES_PER_FRAME: u8 = 154;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PpuMode {
    HBlank,
    VBlank,
    OamScan,
    Draw,
}

/// Represents the PPU state.
pub struct Ppu {
    curr_scanline_dot: u32,
    total_frames_drawn: u64,
    dma: Dma,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            curr_scanline_dot: 0,
            total_frames_drawn: 0,
            dma: Dma::new(),
        }
    }

    pub fn total_frames_drawn(&self) -> u64 {
        self.total_frames_drawn
    }

    pub fn dma_mut(&mut self) -> &mut Dma {
        &mut self.dma
    }
}

/// Advances the PPU state by 1 M-Cycle.
pub fn update_ppu(sys: &mut Sys) {
    // Advance by 1 M-Cycle (4 dots).
    for _ in 0..4 {
        update(sys);
    }
    update_dma(sys);
}

fn update(sys: &mut Sys) {
    let mut ly = sys.mem.io_regs.get(IoReg::Ly);

    let prev_mode = get_mode(sys.ppu.curr_scanline_dot, ly);

    sys.ppu.curr_scanline_dot += 1;
    if sys.ppu.curr_scanline_dot >= DOTS_PER_SCANLINE {
        sys.ppu.curr_scanline_dot = 0;
        ly += 1;
        if ly >= SCANLINES_PER_FRAME as u8 {
            ly = 0;
            sys.ppu.total_frames_drawn += 1;
        }

        enter_scanline(sys, ly);
    }

    let next_mode = get_mode(sys.ppu.curr_scanline_dot, ly);

    if prev_mode != next_mode {
        enter_mode(sys, next_mode);
    }
}

fn get_mode(dot: u32, scanline: u8) -> PpuMode {
    if scanline >= 144 {
        return PpuMode::VBlank;
    } else if dot < 80 {
        return PpuMode::OamScan;
    } else if dot - 80 < 172 {
        return PpuMode::Draw;
    } else {
        return PpuMode::HBlank;
    }
}

fn enter_scanline(sys: &mut Sys, scanline: u8) {
    // Update LY
    let ly = scanline;
    sys.mem.io_regs.set(IoReg::Ly, ly);

    // Update STAT.LYC==LY flag.
    let lyc = sys.mem.io_regs.get(IoReg::Lyc);
    let stat = sys.mem.io_regs.mut_(IoReg::Stat, |stat| {
        let lyc_ly: u8 = (lyc == ly).into();
        set_bit8(stat, 2, lyc_ly);
    });

    let lyc_ly_sel = bit8(&stat, 6) == 1;
    let lyc_ly = bit8(&stat, 2) == 1;

    if lyc_ly_sel && lyc_ly {
        request_interrupt(sys, InterruptType::Stat);
    }
}

fn enter_mode(sys: &mut Sys, mode: PpuMode) {
    // Perform specific actions for mode.
    match mode {
        PpuMode::VBlank => {
            //render_screen(sys);
            sys.is_render_pending = true;
            request_interrupt(sys, InterruptType::VBlank);
        }
        PpuMode::Draw => {
            let ly = sys.mem.io_regs.get(IoReg::Ly);
            render_scanline(sys, ly, VIEWPORT_ORG);
        }
        _ => {}
    }

    // Update the PPU mode indicator bits (1:0)
    let stat = sys.mem.io_regs.mut_(IoReg::Stat, |stat| {
        *stat &= 0b1111_1100;
        *stat |= mode as u8;
    });

    // Request an interrupt if mode request condition is met.
    let stat_mode_flag_idx = match mode {
        PpuMode::HBlank => 3,
        PpuMode::VBlank => 4,
        PpuMode::OamScan => 5,
        _ => {
            return;
        }
    };
    let is_stat_mode_flag_set = bit8(&stat, stat_mode_flag_idx) == 1;
    if is_stat_mode_flag_set {
        request_interrupt(sys, InterruptType::Stat);
    }
}

pub fn print_ppu(sys: &Sys) {
    let dot = sys.ppu.curr_scanline_dot;
    let ly = sys.mem.io_regs.get(IoReg::Ly);
    let mode = get_mode(dot, ly);

    println!("PPU:");
    println!("  curr mode = {:?}", mode);
    println!("  scanline dots = {}", dot);
    println!("  LY = {}", ly);
    println!("  frames drawn = {}", sys.ppu.total_frames_drawn);
}
