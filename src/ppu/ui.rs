use macroquad::color::BLACK;
use xf::{
    mq::draw::draw_rect,
    num::{
        irect::ir,
        ivec2::{i2, IVec2},
    },
};

use crate::{consts::P8, sys::Sys};

use super::{
    consts::{TILE_DATA_BANK_SIZE, TILE_DATA_ORG, TILE_MAP_ORG, VIEWPORT_P8_SIZE},
    debug_draw::render_screen,
    lcdc::LcdcState,
    text::draw_text,
};

pub fn render_ui(sys: &mut Sys) {
    draw_rect(ir(IVec2::ZERO, i2(VIEWPORT_P8_SIZE.x + 1, 1) * P8), BLACK);
    draw_rect(ir(IVec2::ZERO, i2(1, VIEWPORT_P8_SIZE.y + 1) * P8), BLACK);
    let game_title = sys.mem.cart.header().title();
    draw_text(game_title, i2(1, 0) * P8);

    let lcdc = LcdcState::from(sys);
    let bg_tile_map_area = lcdc.bg_tile_map_area();
    draw_text(
        format!("0x{:0>4X} BG MAP", bg_tile_map_area),
        TILE_MAP_ORG - i2(0, 8),
    );

    draw_text("0x8000 TILES", TILE_DATA_ORG - i2(0, 8));
    draw_text("0x8800", TILE_DATA_ORG + i2(0, TILE_DATA_BANK_SIZE.y));
    draw_text(
        "0x9000",
        TILE_DATA_ORG + i2(0, 2 * TILE_DATA_BANK_SIZE.y) + i2(0, 8),
    );

    render_screen(sys);
}
