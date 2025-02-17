use xf::num::ivec2::{i2, IVec2};

use crate::{
    mem::{io_regs::IoReg, sections::Addr},
    sys::Sys,
    util::math::bit8,
};

use super::{
    consts::{
        OAM_ADDR_FE00, OAM_OBJ_SIZE, TILE_DATA_ADDR_8000, TILE_DATA_TILE_SIZE, TILE_MAP_P8_SIZE,
    },
    lcdc::LcdcState,
    palette::Palette,
    render_util::{draw_pixel, get_tile_map_addr, tile_data_idx_to_addr},
};

pub fn render_scanline(sys: &mut Sys, ly: u8, org: IVec2) {
    let lcdc = LcdcState::from(sys);

    let scx = sys.mem.io_regs.get(IoReg::Scx);
    let scy = sys.mem.io_regs.get(IoReg::Scy);

    let src_y = u8::wrapping_add(ly, scy);

    let bgp = Palette::from_reg(sys, IoReg::Bgp);

    // Draw background
    if lcdc.bg_window_enable {
        for x in 0..160 {
            let src_x = u8::wrapping_add(scx, x);
            let color_id = sample_pixel_from_bg_tilemap(sys, src_x, src_y);
            draw_pixel::<false>(i2(x as i32, ly as i32) + org, &bgp, color_id);
        }
    }

    // Draw objects
    if lcdc.obj_enable {
        for obj_idx in 0..40u8 {
            try_draw_obj_row(sys, obj_idx, ly, org);
        }
    }

    // Draw window
    if lcdc.bg_window_enable && lcdc.window_enable {
        for x in 0..160 {
            if let Some(color_id) = sample_pixel_from_window_tilemap(sys, x, ly) {
                draw_pixel::<false>(i2(x as i32, ly as i32) + org, &bgp, color_id);
            }
        }
    }
}

#[inline]
fn sample_pixel_from_bg_tilemap(sys: &Sys, x: u8, y: u8) -> u8 {
    let lcdc = LcdcState::from(sys);

    return sample_pixel_from_tilemap(
        sys,
        x,
        y,
        lcdc.bg_tile_map_area_is_9c00,
        lcdc.bg_window_tile_data_area_is_8000,
    );
}

#[inline]
fn sample_pixel_from_window_tilemap(sys: &Sys, x: u8, y: u8) -> Option<u8> {
    let lcdc = LcdcState::from(sys);
    let wx = sys.mem.io_regs.get(IoReg::Wx);
    if !(0..=166).contains(&wx) {
        return None;
    }
    let wy = sys.mem.io_regs.get(IoReg::Wy);
    if !(0..=143).contains(&wy) {
        return None;
    }

    if wy > y {
        return None;
    }

    let x = u8::saturating_sub(x, wx);
    let y = u8::saturating_sub(y, wy);

    let pixel = sample_pixel_from_tilemap(
        sys,
        x,
        y,
        lcdc.window_tile_map_area_is_9c00,
        lcdc.bg_window_tile_data_area_is_8000,
    );
    return Some(pixel);
}

#[inline]
fn sample_pixel_from_tilemap(
    sys: &Sys,
    x: u8,
    y: u8,
    is_map_mode_9c00: bool,
    is_data_mode_8000: bool,
) -> u8 {
    let tile_map_start_addr = get_tile_map_addr(is_map_mode_9c00);

    let tile_x_idx = x / 8;
    let tile_y_idx = y / 8;
    let map_idx = (tile_y_idx as u16 * TILE_MAP_P8_SIZE.x as u16) + tile_x_idx as u16;
    let map_addr = tile_map_start_addr + map_idx;

    let data_idx = sys.mem.read(map_addr);
    let data_addr = tile_data_idx_to_addr(data_idx as u16, is_data_mode_8000);

    let pixel_x_bit = 7 - (x % 8);
    let pixel_y = y % 8;
    let row_lowers_addr = data_addr + (pixel_y as u16 * 2);
    let row_uppers_addr = row_lowers_addr + 1;

    let lo = bit8(&sys.mem.read(row_lowers_addr), pixel_x_bit);
    let hi = bit8(&sys.mem.read(row_uppers_addr), pixel_x_bit);

    return (hi << 1) | lo;
}

#[inline]
fn try_draw_obj_row(sys: &Sys, obj_idx: u8, ly: u8, org: IVec2) {
    let lcdc = LcdcState::from(sys);

    let obj_addr = OAM_ADDR_FE00 + (OAM_OBJ_SIZE * obj_idx as Addr);
    let y_pos = sys.mem.read(obj_addr + 0);
    let x_pos = sys.mem.read(obj_addr + 1);
    let mut tile_idx = sys.mem.read(obj_addr + 2) as u16;
    let attrs = sys.mem.read(obj_addr + 3);

    let obj_h = if lcdc.obj_size_is_8x16 { 16 } else { 8 };
    if !(y_pos..(y_pos + obj_h)).contains(&ly) {
        return;
    }

    //let priority = bit8(&attrs, 7) == 1;
    let y_flip = bit8(&attrs, 6) == 1;
    let x_flip = bit8(&attrs, 5) == 1;
    let palette_reg = if bit8(&attrs, 4) == 0 {
        IoReg::Obp0
    } else {
        IoReg::Obp1
    };

    let palette = Palette::from_reg(sys, palette_reg);

    let mut pixel_y = ly - y_pos;
    if pixel_y >= 8 {
        tile_idx += 1;
        pixel_y -= 8;
    }

    let tile_data_addr = (tile_idx) * TILE_DATA_TILE_SIZE + TILE_DATA_ADDR_8000;

    for x in 0..8 {
        let pixel_x_bit = if x_flip { x % 8 } else { 7 - (x % 8) };
        let row_lowers_addr = tile_data_addr + (pixel_y as u16 * 2);
        let row_uppers_addr = row_lowers_addr + 1;

        let lo = bit8(&sys.mem.read(row_lowers_addr), pixel_x_bit);
        let hi = bit8(&sys.mem.read(row_uppers_addr), pixel_x_bit);

        let color_id = (hi << 1) | lo;
        draw_pixel::<true>(
            i2(u8::wrapping_add(x_pos, x) as i32 - 8, ly as i32 - 16) + org,
            &palette,
            color_id,
        );
    }
}
