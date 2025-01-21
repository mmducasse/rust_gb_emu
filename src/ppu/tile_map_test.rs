use std::mem::transmute;

use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY, WHITE, YELLOW};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::{ir, rect},
        ivec2::{i2, IVec2},
    },
};

use crate::{
    consts::P8,
    mem::{
        io_regs::IoReg,
        map::{Addr, MemSection},
    },
    sys::Sys,
    util::math::bit8,
};

const TILE_SIZE: IVec2 = P8;
const TILE_MAP_P8_SIZE: IVec2 = i2(32, 32);
const TILE_MAP_1_ADDR: Addr = 0x9800;
const TILE_MAP_2_ADDR: Addr = 0x9C00;

pub const SCREEN_P8_SIZE: IVec2 = i2(32, 32);
pub const SCREEN_SIZE: IVec2 = IVec2::mul(SCREEN_P8_SIZE, P8);

pub fn draw_bg_tile_map(sys: &Sys) {
    let tile_map_1 = TILE_MAP_1_ADDR..TILE_MAP_2_ADDR;

    for i in 0..TILE_MAP_P8_SIZE.product() {
        let x = i % TILE_MAP_P8_SIZE.x;
        let y = i / TILE_MAP_P8_SIZE.x;
        let addr = (i as u16) + TILE_MAP_1_ADDR;

        // if (x + y) % 2 == 0 {
        //     draw_rect(rect(x * 8, y * 8, 8, 8), YELLOW);
        // }
        draw_tile_from_map(sys, i2(x, y), addr);
    }
}

fn draw_tile_from_map(sys: &Sys, pos: IVec2, map_addr: Addr) {
    let lcdc = sys.mem_get(IoReg::Lcdc);
    let mode_8000 = false; // bit8(&lcdc, 4) == 0;
    let tile_idx = sys.mem_get(map_addr);
    //println!("  [{:0>4X}] => {:0>2X}", map_addr, tile_idx);
    let tile_data_addr = if mode_8000 {
        (tile_idx as u16) * 16 + 0x8000
    } else {
        let s_tile_idx = unsafe { transmute::<u8, i8>(tile_idx) };
        ((tile_idx as i32) * 16 + 0x9000) as u16
    };

    //println!(" tile_data_addr = {:0>4X}", tile_data_addr);
    let addr = (tile_data_addr - MemSection::Vram.start_addr()) as usize;
    let bytes = &sys.mem.vram.as_slice()[addr..(addr + 16)];

    let org = pos * P8;
    draw_tile(bytes, org);
}

fn draw_tile(bytes: &[u8], org: IVec2) {
    for pos in rect(0, 0, 8, 8).iter() {
        let idx = (pos.y * 2) as usize;
        let bit = 7 - pos.x;
        let lower = bit8(&bytes[idx], bit as u8);
        let upper = bit8(&bytes[idx + 1], bit as u8);

        let color_id = (upper << 1) | lower;

        draw_pixel(pos + org, color_id);
    }
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
