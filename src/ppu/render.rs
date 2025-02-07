use std::mem::transmute;

use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY, WHITE};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::{
    mem::{io_regs::IoReg, sections::Addr},
    sys::Sys,
    util::math::bit8,
};

use super::{
    consts::{TILE_MAP_ADDR_9800, TILE_MAP_ADDR_9C00, TILE_MAP_P8_SIZE},
    lcdc::LcdcState,
};

pub fn render_scanline(sys: &mut Sys, ly: u8, org: IVec2) {
    let scx = sys.mem.io_regs.get(IoReg::Scx);
    let scy = sys.mem.io_regs.get(IoReg::Scy);

    let src_y = u8::wrapping_add(ly, scy);

    for x in 0..160 {
        let src_x = u8::wrapping_add(scx, x);
        let color_id = sample_pixel_from_tilemap(sys, src_x, src_y);
        draw_pixel(i2(x as i32, ly as i32) + org, color_id);
    }
}

fn sample_pixel_from_tilemap(sys: &Sys, x: u8, y: u8) -> u8 {
    let lcdc = LcdcState::from(sys);
    let tile_map_start_addr = if lcdc.bg_tile_map_area_is_9c00 {
        TILE_MAP_ADDR_9C00
    } else {
        TILE_MAP_ADDR_9800
    };

    let tile_x_idx = x / 8;
    let tile_y_idx = y / 8;
    let tile_idx = (tile_y_idx as u16 * TILE_MAP_P8_SIZE.x as u16) + tile_x_idx as u16;
    let map_addr = tile_map_start_addr + tile_idx;

    let tile_idx = sys.mem.read(map_addr);
    let tile_data_addr = if lcdc.bg_window_tile_data_area_is_8000 {
        (tile_idx as u16) * 16 + 0x8000
    } else {
        let s_tile_idx = unsafe { transmute::<u8, i8>(tile_idx) };
        ((tile_idx as i32) * 16 + 0x9000) as u16
    };

    let pixel_x = 7 - (x % 8);
    let pixel_y = y % 8;
    let row_lowers_addr = tile_data_addr + (pixel_y as u16 * 2);
    let row_uppers_addr = row_lowers_addr + 1;

    let lo = bit8(&sys.mem.read(row_lowers_addr), pixel_x);
    let hi = bit8(&sys.mem.read(row_uppers_addr), pixel_x);

    return (hi << 1) | lo;
}

fn draw_pixel(pos: IVec2, color_id: u8) {
    draw_rect(ir(pos, i2(1, 1)), get_color(color_id));
}

#[inline]
fn get_color(color_id: u8) -> Color {
    return match color_id {
        0b00 => WHITE,
        0b01 => LIGHTGRAY,
        0b10 => DARKGRAY,
        0b11 => BLACK,
        _ => unreachable!(),
    };
}
