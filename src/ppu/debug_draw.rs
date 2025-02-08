use std::{mem::transmute, ops::Add};

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
    other::joypad::draw_joypad_state,
    sys::Sys,
    util::{draw::draw_empty_rect, math::bit8},
};

use super::{consts::*, lcdc::LcdcState};

pub fn render_screen(sys: &mut Sys) {
    let lcdc = LcdcState::from(&sys);

    if !lcdc.ppu_enable {
        return;
    }

    // Render background
    if lcdc.bg_window_enable {
        render_background(sys, TILE_MAP_ORG);
    }

    // Render objects
    if lcdc.obj_enable {
        render_objects(sys, TILE_MAP_ORG);
    }

    // Render debugging info.
    render_tile_data(sys, TILE_DATA_ORG);
    draw_joypad_state(JOYPAD_ORG);

    // Draw moving dot to indicate frame rate.
    let frame = sys.ppu.debug_frames_drawn() as i32;
    let x = frame % TILE_MAP_SIZE.x;
    let y = (frame % TILE_MAP_SIZE.product()) / TILE_MAP_SIZE.x;
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
        draw_tile_from_map(sys, i2(x, y), addr, org);
    }

    // Draw viewport outline.
    let scx = sys.mem.io_regs.get(IoReg::Scx);
    let scy = sys.mem.io_regs.get(IoReg::Scy);
    let viewport_pos = i2(scx as i32, scy as i32);
    let viewport_bounds = ir(viewport_pos, VIEWPORT_P8_SIZE * P8);
    // draw_empty_rect(viewport_bounds.offset_by(org), YELLOW);
    // draw_empty_rect(
    //     viewport_bounds
    //         .offset_by(i2(-TILE_MAP_SIZE.x, 0))
    //         .offset_by(org),
    //     YELLOW,
    // );
    // draw_empty_rect(
    //     viewport_bounds
    //         .offset_by(i2(0, -TILE_MAP_SIZE.y))
    //         .offset_by(org),
    //     YELLOW,
    // );
    // draw_empty_rect(
    //     viewport_bounds
    //         .offset_by(i2(-TILE_MAP_SIZE.x, -TILE_MAP_SIZE.y))
    //         .offset_by(org),
    //     YELLOW,
    // );
}

fn render_objects(sys: &Sys, org: IVec2) {
    let scx = sys.mem.io_regs.get(IoReg::Scx);
    let scy = sys.mem.io_regs.get(IoReg::Scy);
    let viewport_pos = i2(scx as i32, scy as i32);

    for idx in 0..40 {
        let addr = 0xFE00 + (idx * 4);
        let y = sys.mem.read(addr + 0);
        let x = sys.mem.read(addr + 1);
        let tile_idx = sys.mem.read(addr + 2);
        let attrs = sys.mem.read(addr + 3);

        //let priority = bit8(&attrs, 6);
        let y_flip = bit8(&attrs, 6);
        let x_flip = bit8(&attrs, 5);
        let palette = bit8(&attrs, 4);
        let bank = bit8(&attrs, 3);

        let map_addr = if bank == 0 {
            0x8000 + tile_idx as u16
        } else {
            0x8800 + tile_idx as u16
        };

        let pos = i2(x as i32, y as i32);
        draw_tile_from_map(sys, viewport_pos + pos + org, map_addr, org);
    }
}

fn draw_tile_from_map(sys: &Sys, pos: IVec2, map_addr: Addr, org: IVec2) {
    let lcdc = sys.mem.io_regs.get(IoReg::Lcdc);
    let mode_8000 = bit8(&lcdc, 4) == 1;
    let tile_idx = sys.mem.read(map_addr);
    //println!("  [{:0>4X}] => {:0>2X}", map_addr, tile_idx);
    let tile_data_addr = if mode_8000 {
        (tile_idx as u16) * 16 + 0x8000
    } else {
        if tile_idx < 128 {
            (tile_idx as u16) * 16 + 0x9000
        } else {
            ((tile_idx + 128) as u16) * 16 + 0x8800
        }
    };

    //println!(" tile_data_addr = {:0>4X}", tile_data_addr);
    let addr = (tile_data_addr - MemSection::Vram.start_addr()) as usize;
    let bytes = &sys.mem.vram.as_slice()[addr..(addr + 16)];

    let org = pos * P8 + org;
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
