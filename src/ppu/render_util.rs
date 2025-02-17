use macroquad::color::{Color, BLACK, DARKGRAY, LIGHTGRAY, WHITE};
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::mem::Addr;

use super::{
    consts::{
        TILE_DATA_ADDR_8000, TILE_DATA_ADDR_8800, TILE_DATA_ADDR_9000, TILE_DATA_TILE_SIZE,
        TILE_MAP_ADDR_9800, TILE_MAP_ADDR_9C00,
    },
    palette::Palette,
};

#[inline]
pub fn get_tile_map_addr(is_map_mode_9c00: bool) -> Addr {
    let tile_map_start_addr = if is_map_mode_9c00 {
        TILE_MAP_ADDR_9C00
    } else {
        TILE_MAP_ADDR_9800
    };

    return tile_map_start_addr;
}

#[inline]
pub fn tile_data_idx_to_addr(data_idx: u16, is_data_mode_8000: bool) -> Addr {
    let data_addr = if is_data_mode_8000 {
        data_idx * TILE_DATA_TILE_SIZE + TILE_DATA_ADDR_8000
    } else if data_idx < 128 {
        data_idx * TILE_DATA_TILE_SIZE + TILE_DATA_ADDR_9000
    } else {
        (data_idx - 128) * TILE_DATA_TILE_SIZE + TILE_DATA_ADDR_8800
    };

    return data_addr;
}

#[inline]
pub fn draw_pixel<const TRANSPARENT: bool>(pos: IVec2, palette: &Palette, color_id: u8) {
    if TRANSPARENT && (color_id == 0) {
        return;
    }
    let color = get_color(palette.map(color_id));
    draw_rect(ir(pos, i2(1, 1)), color);
}

#[inline]
fn get_color(color_value: u8) -> Color {
    return match color_value {
        0b00 => WHITE,
        0b01 => LIGHTGRAY,
        0b10 => DARKGRAY,
        0b11 => BLACK,
        _ => unreachable!(),
    };
}
