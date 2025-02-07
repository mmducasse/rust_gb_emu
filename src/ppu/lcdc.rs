use crate::{mem::io_regs::IoReg, sys::Sys, util::math::bit8};

pub struct LcdcState {
    pub ppu_enable: bool,
    pub window_tile_map_area_is_9c00: bool,
    pub window_enable: bool,
    pub bg_window_tile_data_area_is_8000: bool,
    pub bg_tile_map_area_is_9c00: bool,
    pub obj_size_is_8x16: bool,
    pub obj_enable: bool,
    pub bg_window_enable: bool,
}

impl LcdcState {
    pub fn from(sys: &Sys) -> Self {
        let lcdc = sys.mem.io_regs.get(IoReg::Lcdc);

        Self {
            ppu_enable: bit8(&lcdc, 7) == 1,
            window_tile_map_area_is_9c00: bit8(&lcdc, 6) == 1,
            window_enable: bit8(&lcdc, 5) == 1,
            bg_window_tile_data_area_is_8000: bit8(&lcdc, 4) == 1,
            bg_tile_map_area_is_9c00: bit8(&lcdc, 3) == 1,
            obj_size_is_8x16: bit8(&lcdc, 2) == 1,
            obj_enable: bit8(&lcdc, 1) == 1,
            bg_window_enable: bit8(&lcdc, 0) == 1,
        }
    }
}
