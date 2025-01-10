use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    sys::Sys,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PpuMode {
    HBlank,
    VBlank,
    OamScan,
    Draw,
}

impl PpuMode {
    // todo implement variable draw time
    pub fn typical_duration(self) -> u32 {
        match self {
            PpuMode::HBlank => 204,
            PpuMode::VBlank => 4560,
            PpuMode::OamScan => 80,
            PpuMode::Draw => 172,
        }
    }
}

// "Dot" = 1 system clock period ~ 1 / 4MHz.
// For 144 scanlines
//   Mode 2: OAM Scan: 80 dots
//   Mode 3: Drawing: 172-289 dots
//   Mode 0: HBlank: 376 - (mode 3's duration) dots
// For 10 scanlines
//   Mode 1: VBlank: 4560 dots

pub struct Ppu {
    curr_mode: PpuMode,
    dots_since_mode_start: u32,
    curr_scanline: u32,

    debug_frames_drawn: u32,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            curr_mode: PpuMode::HBlank,
            dots_since_mode_start: 0,
            curr_scanline: 0,
            debug_frames_drawn: 0,
        }
    }

    pub fn update_ppu(sys: &mut Sys, dots: u32) {
        let dots_since_mode_start = sys.ppu.dots_since_mode_start + dots;
        if dots_since_mode_start < sys.ppu.curr_mode.typical_duration() {
            // Stay in current mode.
            sys.ppu.dots_since_mode_start = dots_since_mode_start;
            return;
        } else {
            // Advance to next mode.
            let excess_dots = dots_since_mode_start - sys.ppu.curr_mode.typical_duration();
            sys.ppu.dots_since_mode_start = excess_dots;

            match sys.ppu.curr_mode {
                PpuMode::HBlank => {
                    if sys.ppu.curr_scanline == 143 {
                        request_interrupt(sys, InterruptType::VBlank);
                        sys.ppu.curr_mode = PpuMode::VBlank;
                    } else {
                        sys.ppu.curr_mode = PpuMode::OamScan;
                    }
                    sys.ppu.curr_scanline += 1;
                }
                PpuMode::VBlank => {
                    sys.ppu.curr_mode = PpuMode::OamScan;
                    sys.ppu.curr_scanline = 0;
                    sys.ppu.debug_frames_drawn += 1;
                }
                PpuMode::OamScan => {
                    sys.ppu.curr_mode = PpuMode::Draw;
                }
                PpuMode::Draw => {
                    sys.ppu.curr_mode = PpuMode::HBlank;
                }
            }
        }
    }

    pub fn print(&self) {
        println!("PPU:");
        println!("  curr mode = {:?}", self.curr_mode);
        println!("  dots since mode start = {}", self.dots_since_mode_start);
        println!("  curr scanline = {}", self.curr_scanline);
        println!("  frames drawn = {}", self.debug_frames_drawn);
    }
}
