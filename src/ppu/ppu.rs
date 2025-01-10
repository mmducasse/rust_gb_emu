use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    mem::io_regs::IoReg,
    sys::Sys,
    util::math::{bit8, set_bit8},
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
    debug_frames_drawn: u32,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            curr_mode: PpuMode::VBlank,
            dots_since_mode_start: 0,
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

            let curr_scanline = sys.mem_get(IoReg::Ly);

            match sys.ppu.curr_mode {
                PpuMode::HBlank => {
                    Self::set_scanline(sys, curr_scanline + 1);
                    if curr_scanline == 143 {
                        Self::enter_mode(sys, PpuMode::VBlank);
                        request_interrupt(sys, InterruptType::VBlank);
                    } else {
                        Self::enter_mode(sys, PpuMode::OamScan);
                    }
                }
                PpuMode::VBlank => {
                    Self::set_scanline(sys, 0);
                    Self::enter_mode(sys, PpuMode::OamScan);
                    sys.ppu.debug_frames_drawn += 1;
                }
                PpuMode::OamScan => {
                    Self::enter_mode(sys, PpuMode::Draw);
                }
                PpuMode::Draw => {
                    Self::enter_mode(sys, PpuMode::HBlank);
                }
            }
        }
    }

    fn enter_mode(sys: &mut Sys, mode: PpuMode) {
        sys.ppu.curr_mode = mode;

        // Update the PPU mode indicator bits (1:0)
        let stat = sys.mem_mut(IoReg::Stat, |stat| {
            *stat &= 0b1111_1100;
            *stat |= mode as u8;
        });

        // Request an interrupt, if mode request condition is met.
        let mode_req_flag_idx = match mode {
            PpuMode::HBlank => 3,
            PpuMode::VBlank => 4,
            PpuMode::OamScan => 5,
            _ => {
                return;
            }
        };
        let is_req_flag_set = bit8(&stat, mode_req_flag_idx) == 1;
        if is_req_flag_set {
            request_interrupt(sys, InterruptType::Stat);
        }
    }

    fn set_scanline(sys: &mut Sys, next: u8) {
        let ly = sys.mem_mut(IoReg::Ly, |ly| {
            *ly = next;
        });

        let lyc = sys.mem_get(IoReg::Lyc);
        let eq = ly == lyc;

        let stat = sys.mem_mut(IoReg::Stat, |stat| {
            set_bit8(stat, 2, eq.into());
        });

        let is_lyc_req_flag_set = bit8(&stat, 6) == 1;
        if is_lyc_req_flag_set && eq {
            request_interrupt(sys, InterruptType::Stat);
        }
    }

    pub fn print(sys: &Sys) {
        let ly = sys.mem_get(IoReg::Ly);

        println!("PPU:");
        println!("  curr mode = {:?}", sys.ppu.curr_mode);
        println!(
            "  dots since mode start = {}",
            sys.ppu.dots_since_mode_start
        );
        println!("  LY = {}", ly);
        println!("  frames drawn = {}", sys.ppu.debug_frames_drawn);
    }
}
