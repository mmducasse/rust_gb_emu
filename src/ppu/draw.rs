use std::mem::transmute;

use macroquad::color::{Color, BLACK, BLUE, DARKGRAY, GREEN, LIGHTGRAY, RED, WHITE, YELLOW};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::{ir, rect, IRect},
        ivec2::{i2, IVec2},
    },
};

use crate::{
    consts::P8,
    mem::{
        io_regs::IoReg,
        sections::{Addr, MemSection},
    },
    other::joypad::{draw_joypad_state, DRAW_INPUTS_ORG},
    sys::Sys,
    util::{draw::draw_empty_rect, math::bit8},
};

use super::consts::*;

struct LcdcState {
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

pub fn render_screen(sys: &mut Sys) {
    let lcdc = LcdcState::from(&sys);

    if !lcdc.ppu_enable {
        return;
    }

    render_tile_data(sys, i2(TILE_MAP_P8_SIZE.x * 8, 0));

    // Render background
    if lcdc.bg_window_enable {
        render_background(sys, IVec2::ZERO);
    }

    // Render objects
    if lcdc.obj_enable {
        
    }

    // Render window
    if lcdc.bg_window_enable && lcdc.window_enable {
        render_window(sys, IVec2::ZERO);
    }

    draw_joypad_state(DRAW_INPUTS_ORG);

    // Draw moving dot to indicate frame rate.
    let frame = sys.ppu.debug_frames_drawn() as i32;
    let x = frame % TILE_MAP_SIZE.x;
    let y = (frame % TILE_MAP_SIZE.y) / TILE_MAP_SIZE.x;
    draw_rect(ir(i2(x, y), i2(1, 1)), BLUE)
}

pub fn render_tile_data(sys: &Sys, org: IVec2) {
    // let max = SCREEN_SIZE.x;
    // for i in 0..max {
    //     draw_pixel(i2(i, i), 0b01);
    // }

    let mut i = 0;
    let range = 0x8000..0x9800;
    for addr in range.clone().step_by(16) {
        let x = i % TILE_DATA_P8_SIZE.x;
        let y = i / TILE_DATA_P8_SIZE.x;

        let rel_addr = addr - range.start;
        let bytes = &sys.mem.vram.as_slice()[rel_addr..(rel_addr + 16)];

        // if sum_slice(bytes) == 0 {
        //     continue;
        // }
        i += 1;

        // let block_color = if y < 8 {
        //     RED
        // } else if y < 16 {
        //     BLUE
        // } else {
        //     YELLOW
        // };
        // draw_rect(rect(org.x + x * 8, org.y + y * 8, 8, 8), block_color);

        draw_tile(bytes, org + i2(x * 8, y * 8));
    }

    // Draw sections
    let size = (TILE_DATA_P8_SIZE * P8) / i2(1, 3);
    draw_empty_rect(rect(0, 0, size.x, size.y).offset_by(org), BLUE);
    draw_empty_rect(rect(0, size.y, size.x, size.y).offset_by(org), BLUE);
    draw_empty_rect(rect(0, 2 * size.y, size.x, size.y).offset_by(org), BLUE);

    // let offset = 0x9000; // MemSection::Vram.start_addr();
    // for idx in 0..MemSection::Vram.size() {
    //     let data = sys.rd_mem(offset + idx);
    //     println!("[{:0>4X}]: {:0>2X}", idx, data);
    // }
}

pub fn render_background(sys: &Sys, org: IVec2) {
    let lcdc = LcdcState::from(sys);
    let tile_map_start_addr = if lcdc.bg_tile_map_area_is_9c00 {
        TILE_MAP_ADDR_9C00
    } else {
        TILE_MAP_ADDR_9800
    };

    for i in 0..TILE_MAP_P8_SIZE.product() {
        let x = i % TILE_MAP_P8_SIZE.x;
        let y = i / TILE_MAP_P8_SIZE.x;
        let addr = (i as u16) + tile_map_start_addr;

        // if (x + y) % 2 == 0 {
        //     draw_rect(rect(x * 8, y * 8, 8, 8), YELLOW);
        // }
        draw_tile_from_map(sys, i2(x, y), addr);
    }

    // Draw viewport outline.
    let scx = sys.mem.io_regs.get(IoReg::Scx);
    let scy = sys.mem.io_regs.get(IoReg::Scy);
    let viewport_pos = i2(scx as i32, scy as i32);
    let viewport_bounds = ir(viewport_pos, VIEWPORT_P8_SIZE * P8);
    draw_empty_rect(viewport_bounds, YELLOW);
    draw_empty_rect(viewport_bounds.offset_by(TILE_MAP_P8_SIZE * -8), YELLOW);
}

pub fn render_window(sys: &Sys, org: IVec2) {
    let lcdc = LcdcState::from(sys);
    let tile_map_start_addr = if lcdc.window_tile_map_area_is_9c00 {
        TILE_MAP_ADDR_9C00
    } else {
        TILE_MAP_ADDR_9800
    };

    for i in 0..TILE_MAP_P8_SIZE.product() {
        let x = i % TILE_MAP_P8_SIZE.x;
        let y = i / TILE_MAP_P8_SIZE.x;
        let addr = (i as u16) + tile_map_start_addr;

        // if (x + y) % 2 == 0 {
        //     draw_rect(rect(x * 8, y * 8, 8, 8), YELLOW);
        // }
        draw_tile_from_map(sys, i2(x, y), addr);
    }

    // Draw window outline.
    let wx = sys.mem.io_regs.get(IoReg::Scx);
    let wy = sys.mem.io_regs.get(IoReg::Scy);
    let window_pos = i2(wx as i32, wy as i32);
    let window_bounds = ir(window_pos, VIEWPORT_P8_SIZE * P8);
    draw_empty_rect(window_bounds, GREEN);
}

fn draw_tile_from_map(sys: &Sys, pos: IVec2, map_addr: Addr) {
    let lcdc = sys.mem.io_regs.get(IoReg::Lcdc);
    let mode_8000 = bit8(&lcdc, 4) == 1;
    let tile_idx = sys.mem.read(map_addr);
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
