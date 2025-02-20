use macroquad::color::BLACK;
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::{consts::P8, other::joypad::draw_joypad_state, sys::Sys};

use super::{
    consts::{
        JOYPAD_ORG, TILE_DATA_BLOCK_DRAW_P8_SIZE, TILE_DATA_BLOCK_DRAW_SIZE, TILE_DATA_ORG,
        TILE_MAP_ORG, VIEWPORT_P8_SIZE,
    },
    lcdc::LcdcState,
    render_mem::{render_scroll_view_area, render_tile_data_block, render_tile_map},
    render_util::get_tile_map_addr,
    text::draw_text,
};

const SHOW_SCROLL_AREA_OUTLINE: bool = false;

pub fn render_ui(sys: &mut Sys) {
    let lcdc = LcdcState::from(sys);

    // Viewport.
    draw_rect(ir(IVec2::ZERO, i2(VIEWPORT_P8_SIZE.x + 1, 1) * P8), BLACK);
    draw_rect(ir(IVec2::ZERO, i2(1, VIEWPORT_P8_SIZE.y + 1) * P8), BLACK);
    draw_rect(
        ir(
            i2(VIEWPORT_P8_SIZE.x + 1, 0) * P8,
            i2(1, VIEWPORT_P8_SIZE.y + 1) * P8,
        ),
        BLACK,
    );
    draw_rect(
        ir(
            i2(0, VIEWPORT_P8_SIZE.y + 1) * P8,
            i2(VIEWPORT_P8_SIZE.x + 1, 1) * P8,
        ),
        BLACK,
    );
    let game_title = sys.mem.cart.header().title();
    draw_text(game_title, i2(1, 0) * P8);

    // Joypad.
    draw_joypad_state(JOYPAD_ORG);

    if !sys.options.show_debug_views {
        return;
    }

    // Background tilemap view.
    let is_showing_win = sys.emu.show_win_map;
    let tile_map_area_is_9c00 = if is_showing_win {
        lcdc.window_tile_map_area_is_9c00
    } else {
        lcdc.bg_tile_map_area_is_9c00
    };
    let bg_tile_map_area = get_tile_map_addr(tile_map_area_is_9c00);
    let label = if is_showing_win { "WIN" } else { "BG" };
    draw_text(
        format!("0x{:0>4X} {} MAP", bg_tile_map_area, label),
        TILE_MAP_ORG - i2(0, 8),
    );
    let tile_map_addr = get_tile_map_addr(tile_map_area_is_9c00);
    render_tile_map(sys, tile_map_addr, TILE_MAP_ORG);
    let is_even_frame = sys.ppu.total_frames_drawn() % 2 == 0;
    if SHOW_SCROLL_AREA_OUTLINE && !is_showing_win && is_even_frame {
        render_scroll_view_area(sys, TILE_MAP_ORG);
    }

    // Tile data blocks view.
    draw_text("0x8000 TILES", TILE_DATA_ORG - i2(0, 8));
    render_tile_data_block(sys, 0x8000, TILE_DATA_ORG);
    draw_text("0x8800", TILE_DATA_ORG + i2(0, TILE_DATA_BLOCK_DRAW_SIZE.y));
    render_tile_data_block(
        sys,
        0x8800,
        TILE_DATA_ORG + i2(0, TILE_DATA_BLOCK_DRAW_P8_SIZE.y + 1) * P8,
    );
    draw_text(
        "0x9000",
        TILE_DATA_ORG + i2(0, 2 * TILE_DATA_BLOCK_DRAW_SIZE.y) + i2(0, 8),
    );
    render_tile_data_block(
        sys,
        0x9000,
        TILE_DATA_ORG + i2(0, 2 * TILE_DATA_BLOCK_DRAW_P8_SIZE.y + 2) * P8,
    );
}
