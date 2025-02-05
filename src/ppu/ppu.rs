use crate::{
    cpu::interrupt::{request_interrupt, InterruptType},
    mem::io_regs::IoReg,
    sys::Sys,
    util::math::bit8,
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

pub struct Ppu {
    curr_scanline_dot: u32,
    debug_frames_drawn: u64,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            curr_scanline_dot: 0,
            debug_frames_drawn: 0,
        }
    }

    pub fn debug_frames_drawn(&self) -> u64 {
        self.debug_frames_drawn
    }

    pub fn update_ppu(sys: &mut Sys) {
        let mut ly = sys.mem.io_regs.get(IoReg::Ly);

        let prev_mode = Self::get_mode(sys.ppu.curr_scanline_dot, ly);

        sys.ppu.curr_scanline_dot += 1;
        if sys.ppu.curr_scanline_dot >= DOTS_PER_SCANLINE {
            sys.ppu.curr_scanline_dot = 0;
            ly += 1;
            if ly >= SCANLINES_PER_FRAME as u8 {
                ly = 0;
                sys.ppu.debug_frames_drawn += 1;
            }

            sys.mem.io_regs.set(IoReg::Ly, ly);
        }

        let next_mode = Self::get_mode(sys.ppu.curr_scanline_dot, ly);

        if prev_mode != next_mode {
            Self::enter_mode(sys, next_mode);
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

    fn enter_mode(sys: &mut Sys, mode: PpuMode) {
        // Perform specific actions for mode.
        match mode {
            PpuMode::VBlank => {
                //render_screen(sys);
                sys.is_render_pending = true;
            }
            _ => {}
        }

        // Update the PPU mode indicator bits (1:0)
        let stat = sys.mem.io_regs.mut_(IoReg::Stat, |stat| {
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

    pub fn print(sys: &Sys) {
        let dot = sys.ppu.curr_scanline_dot;
        let ly = sys.mem.io_regs.get(IoReg::Ly);
        let mode = Self::get_mode(dot, ly);

        println!("PPU:");
        println!("  curr mode = {:?}", mode);
        println!("  scanline dots = {}", dot);
        println!("  LY = {}", ly);
        println!("  frames drawn = {}", sys.ppu.debug_frames_drawn);
    }
}
