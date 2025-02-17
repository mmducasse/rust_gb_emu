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
    render_mem::{render_bg_tile_map, render_tile_data_block},
    text::draw_text,
};

pub fn render_ui(sys: &mut Sys) {
    let lcdc = LcdcState::from(sys);

    // Viewport.
    draw_rect(ir(IVec2::ZERO, i2(VIEWPORT_P8_SIZE.x + 1, 1) * P8), BLACK);
    draw_rect(ir(IVec2::ZERO, i2(1, VIEWPORT_P8_SIZE.y + 1) * P8), BLACK);
    let game_title = sys.mem.cart.header().title();
    draw_text(game_title, i2(1, 0) * P8);

    // Background tilemap view.
    let bg_tile_map_area = lcdc.bg_tile_map_area();
    draw_text(
        format!("0x{:0>4X} BG MAP", bg_tile_map_area),
        TILE_MAP_ORG - i2(0, 8),
    );
    render_bg_tile_map(sys, TILE_MAP_ORG);

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

    // Joypad.
    draw_joypad_state(JOYPAD_ORG);
}
