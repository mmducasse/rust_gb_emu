use xf::num::ivec2::i2;

use crate::{consts::P8, sys::Sys};

use super::{
    consts::{TILE_DATA_BANK_SIZE, TILE_DATA_ORG, TILE_MAP_ORG},
    debug_draw::render_screen,
    lcdc::LcdcState,
    text::draw_text,
};

pub fn render_ui(sys: &mut Sys) {
    let game_title = sys.mem.cart.header().title.unwrap_or("".to_owned());
    draw_text(game_title, i2(1, 0) * P8);

    let lcdc = LcdcState::from(sys);
    let bg_tile_map_area = lcdc.bg_tile_map_area();
    draw_text(
        format!("0x{:0>4X}", bg_tile_map_area),
        TILE_MAP_ORG - i2(0, 8),
    );

    draw_text("0x8000", TILE_DATA_ORG - i2(0, 8));
    draw_text("0x8800", TILE_DATA_ORG + i2(0, TILE_DATA_BANK_SIZE.y));
    draw_text(
        "0x9000",
        TILE_DATA_ORG + i2(0, 2 * TILE_DATA_BANK_SIZE.y) + i2(0, 8),
    );

    render_screen(sys);
}
